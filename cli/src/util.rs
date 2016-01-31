use std::{env,io,fs};
use std::io::prelude::*;
use std::process::{Command, Stdio};
use cbor;
use rustc_serialize;
use rustc_serialize::hex::ToHex;
use colorhash256;
use interactor;
use secstr::SecStr;
use ansi_term::Colour::Fixed;
use ansi_term::ANSIStrings;

pub fn menu_cmd() -> Option<Command> {
    env::var_os("FREEPASS_MENU").or(env::var_os("MENU"))
        .map(|s| {
            let mut cmd = Command::new(s);
            cmd.env("FREEPASS_MODE", "MENU");
            cmd
        })
}

pub fn read_password_console() -> SecStr {
    SecStr::new(interactor::read_from_tty(|buf, b, tty| {
        if b == 4 {
            tty.write(b"\r                \r").unwrap();
            return;
        }
        let color_string = if buf.len() < 8 {
            // Make it a bit harder to recover the password by e.g. someone filming how you're entering your password
            // Although if you're entering your password on camera, you're kinda screwed anyway
            b"\rPassword: ~~~~~~".to_vec()
        } else {
            let colors = colorhash256::hash_as_ansi(buf);
            format!("\rPassword: {}",
                ANSIStrings(&[
                    Fixed(colors[0] as u8).paint("~~"),
                    Fixed(colors[1] as u8).paint("~~"),
                    Fixed(colors[2] as u8).paint("~~"),
                ])).into_bytes()
        };
        tty.write(&color_string).unwrap();
    }, true, true).unwrap())
}

pub fn read_password_askpass(mut command: Command) -> SecStr {
    let process = command.stdout(Stdio::piped()).spawn().unwrap();
    let mut result = Vec::new();
    let mut reader = io::BufReader::new(process.stdout.unwrap());
    let size = reader.read_until(b'\n', &mut result).unwrap();
    result.truncate(size - 1);
    SecStr::new(result)
}

pub fn read_password() -> SecStr {
    match env::var_os("FREEPASS_ASKPASS").or(env::var_os("ASKPASS")) {
        Some(s) => read_password_askpass(Command::new(s)),
        None => read_password_console(),
    }
}

pub fn read_text_console(prompt: &str) -> Option<String> {
    let mut tty = fs::OpenOptions::new().read(true).write(true).open("/dev/tty").unwrap();
    tty.write(&format!("\r{}: ", prompt).into_bytes()).unwrap();
    let mut reader = io::BufReader::new(tty);
    let mut input = String::new();
    reader.read_line(&mut input).unwrap();
    input = input.replace("\n", "");
    if input.len() > 0 {
        Some(input)
    } else {
        None
    }
}

pub fn read_text_asktext(mut command: Command, prompt: &str) -> Option<String> {
    match command.env("FREEPASS_MODE", "TEXT")
                 .env("FREEPASS_PROMPT", prompt).output() {
        Ok(ref output) if output.status.success() =>
            Some(String::from_utf8_lossy(&output.stdout).replace("\n", "")),
        _ => None
    }
}

pub fn read_text(prompt: &str) -> Option<String> {
    match env::var_os("FREEPASS_ASKTEXT") {
        Some(s) => read_text_asktext(Command::new(s), prompt),
        None => read_text_console(prompt),
    }
}

pub fn read_yesno(prompt: &str) -> bool {
    loop {
        if let Some(x) = read_text(&format!("{} [y/n]", prompt)) {
            match x.to_lowercase().as_ref() {
                "y" | "yes" | "yeah" | "yep" => return true,
                "n" | "no" | "nope" => return false,
                _ => (),
            }
        }
    }
}

pub fn debug_output<T: rustc_serialize::Encodable>(data: &T, description: &str) {
    let mut e = cbor::Encoder::from_memory();
    e.encode(&[data]).unwrap();
    println!("--- CBOR debug output (http://cbor.me to decode) of {} ---\n{}\n", description, e.into_bytes().to_hex());
}
