use std::fs::OpenOptions;
use std::io::prelude::*;
use std::io::{BufReader};
use std::process::{Command, Stdio};
use colorhash256;
use interactor;
use secstr::SecStr;
use ansi_term::Colour::Fixed;
use ansi_term::ANSIStrings;

pub fn get_password_console() -> SecStr {
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

pub fn get_password_askpass(mut command: Command) -> SecStr {
    let process = command.stdout(Stdio::piped()).spawn().unwrap();
    let mut result = Vec::new();
    let mut reader = BufReader::new(process.stdout.unwrap());
    let size = reader.read_until(b'\n', &mut result).unwrap();
    result.truncate(size - 1);
    SecStr::new(result)
}

pub fn read_text(prompt: &str) -> Option<String> {
    let mut tty = OpenOptions::new().read(true).write(true).open("/dev/tty").unwrap();
    tty.write(&format!("\r{}: ", prompt).into_bytes()).unwrap();
    let mut reader = BufReader::new(tty);
    let mut input = String::new();
    reader.read_line(&mut input).unwrap();
    input = input.replace("\n", "");
    if input.len() > 0 {
        Some(input)
    } else {
        None
    }
}
