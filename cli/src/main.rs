extern crate clap;
//extern crate fuse;
extern crate secstr;
extern crate colorhash256;
extern crate interactor;
extern crate rusterpassword;
extern crate ansi_term;
extern crate hex;
extern crate base64;
extern crate serde;
extern crate serde_cbor;
extern crate csv;
extern crate freepass_core;

mod util;
mod openfile;
mod interact;
mod mergein;

use std::{env, fs, io};
use clap::{Arg, App, SubCommand};
use openfile::*;
use freepass_core::{import, vault::{self, Vault}, output};

fn main() {
    let matches = App::new("freepass")
        .version(env!("CARGO_PKG_VERSION"))
        .author("Val Packett <val@packett.cool>")
        .about("The free password manager for power users")
        .arg(
            Arg::with_name("FILE")
                .short("f")
                .long("file")
                .takes_value(true)
                .help("The vault file to use, by default: $FREEPASS_FILE"),
        )
        .arg(
            Arg::with_name("NAME")
                .short("n")
                .long("name")
                .takes_value(true)
                .help("The user name to use (must be always the same for a vault file!), by default: $FREEPASS_NAME"),
        )
        .arg(
            Arg::with_name("DEBUG").long("debug").help(
                "Enable logging of data structures for debugging (DO NOT USE ON YOUR REAL DATA)",
            ),
        )
        .subcommand(SubCommand::with_name("interact").about("Launches interactive mode"))
        .subcommand(SubCommand::with_name("export").about("Exports all records as CSV (Bitwarden compatible fields)"))
        .subcommand(
            SubCommand::with_name("mergein")
                .about(
                    "Adds entires from a second file (possibly importing from a foreign format) that don't exist in the first file (e.g. to resolve file sync conflicts)",
                )
                .arg(
                    Arg::with_name("IMPORTTYPE")
                        .short("i")
                        .long("import")
                        .takes_value(true)
                        .help(
                            "If you want to import from a foreign file format instead of merging a second freepass vault, the format of that file. Supported: kdbx",
                        ),
                )
                .arg(
                    Arg::with_name("SECONDFILE")
                        .short("F")
                        .long("secondfile")
                        .takes_value(true)
                        .help("The vault file to get additional entries from, by default: $FREEPASS_SECOND_FILE"),
                )
                .arg(
                    Arg::with_name("SECONDNAME")
                        .short("N")
                        .long("secondname")
                        .takes_value(true)
                        .help("The user name to use for the second file, by default: $FREEPASS_SECOND_NAME or the first file name"),
                ),
        )
        .get_matches();

    let file_path = unwrap_for_opt(opt_or_env(&matches, "FILE", "FREEPASS_FILE"), "file");
    let user_name = unwrap_for_opt(opt_or_env(&matches, "NAME", "FREEPASS_NAME"), "name");
    let debug = matches.is_present("DEBUG");

    freepass_core::init();

    // Ensure we can write! Maybe someone somewhere would want to open the vault in read-only mode...
    // But the frustration of trying to save the vault while only having read permissions would be worse.
    let mut open_file = OpenFile::open(file_path.clone(), &user_name, util::read_password(), true);

    if debug {
        util::debug_output(&open_file.vault.data, "Vault");
    }

    match matches.subcommand() {

        ("mergein", submatches_opt) => {
            if let Some(submatches) = submatches_opt {
                let second_file_path = unwrap_for_opt(opt_or_env(submatches, "SECONDFILE", "FREEPASS_SECOND_FILE"), "secondfile");
                let second_vault: Box<vault::Vault> = match submatches.value_of("IMPORTTYPE") {
                    Some("kdbx") => {
                        let mut second_file = match fs::OpenOptions::new().read(true).open(&second_file_path) {
                            Ok(file) => file,
                            Err(ref err) => panic!("Could not open file {}: {}", &second_file_path, err),
                        };
                        Box::new(import::kdbx(&mut second_file, &util::read_password()).expect("Could not read the file as kdbx"))
                    },
                    Some(x) => panic!("Unsupported import format {}", x),
                    None => {
                        let second_user_name = opt_or_env(submatches, "SECONDNAME", "FREEPASS_SECOND_NAME").unwrap_or(user_name);
                        let second_open_file = OpenFile::open(second_file_path, &second_user_name, util::read_password(), false);
                        if debug {
                            util::debug_output(&second_open_file.vault.data, "Second Vault");
                        }
                        Box::new(second_open_file.vault)
                    },
                };
                mergein::merge_in(&mut open_file.vault, &*second_vault);
                open_file.save();
            } else {
                panic!("No options for mergein")
            }
        },

        ("export", _) => {
            let stdout = io::stdout();
            let mut stdout = stdout.lock();
            let mut writer = csv::Writer::from_writer(stdout);
            writer.write_record(&["folder", "favorite", "type", "name", "notes", "fields", "login_uri", "login_username", "login_password", "login_totp"]).unwrap();
            for name in open_file.vault.entry_names() {
                let (entry, _meta) = open_file.vault.get_entry(name).expect("Couldn't read selected entry");
                let username = match entry.fields.get("username").or(entry.fields.get("login")).and_then(|f| output::process_output(name, &open_file.master_key, f).ok()) {
                    Some(output::Output::OpenText(s)) => s,
                    _ => "".to_string(),
                };
                let password = match entry.fields.get("password").and_then(|f| output::process_output(name, &open_file.master_key, f).ok()) {
                    Some(output::Output::PrivateText(s)) => String::from_utf8(Vec::from(s.unsecure())).expect("Couldn't decode UTF-8"),
                    _ => "".to_string(),
                };
                let extras = entry.fields.iter().filter(|(k, _)| *k != "username" && *k != "login" && *k != "password").filter_map(|(k, v)| {
                    match output::process_output(k, &open_file.master_key, v) {
                        Ok(output::Output::OpenText(s)) => Some(format!("{}: {}", k, s)),
                        Ok(output::Output::PrivateText(s)) => Some(format!("{}: {}", k, String::from_utf8(Vec::from(s.unsecure())).expect("Couldn't decode UTF-8"))),
                        _ => None,
                    }
                }).collect::<Vec<_>>().join("\n");
                writer.write_record(&[file_path.to_string(), "".to_string(), "login".to_string(), name.to_string(), "".to_string(), extras, format!("https://{}", name), username, password, "".to_string()]).unwrap();
            }
            writer.flush().unwrap();
        },

        ("interact", _) | _ => interact::interact_entries(&mut open_file, debug),

    }
}

fn opt_or_env(matches: &clap::ArgMatches, opt_name: &str, env_name: &str) -> Option<String> {
    matches
        .value_of(opt_name)
        .map(|x| x.to_owned())
        .or(env::var_os(env_name).and_then(|s| s.into_string().ok()))
}

fn unwrap_for_opt(opt: Option<String>, name: &str) -> String {
    match opt {
        Some(s) => s,
        None => panic!("Option {} not found", name),
    }
}
