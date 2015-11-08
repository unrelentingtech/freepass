#[macro_use] extern crate clap;
extern crate secstr;
extern crate colorhash256;
extern crate interactor;
extern crate rusterpassword;
extern crate sodiumoxide;
extern crate ansi_term;
extern crate freepass_core;

use std::{fs,env,io};
use std::io::prelude::*;
use ansi_term::Colour::Fixed;
use ansi_term::ANSIStrings;
use secstr::*;
use interactor::*;
use rusterpassword::*;
use freepass_core::data::*;

fn opt_or_env(matches: &clap::ArgMatches, opt_name: &str, env_name: &str) -> String {
    match matches.value_of(opt_name).map(|x| x.to_owned()).or(env::var_os(env_name).and_then(|s| s.into_string().ok())) {
        Some(s) => s,
        None => panic!("Option {} or environment variable {} not found", opt_name, env_name)
    }
}

fn main() {
    let matches = clap_app!(freepass =>
        (version: env!("CARGO_PKG_VERSION"))
        (author: "Val Packett <val@packett.cool>")
        (about: "The free password manager for power users")
        (@arg FILE: -f --file +takes_value "Sets the vault file to use, by default: $FREEPASS_FILE")
        (@arg NAME: -n --name +takes_value "Sets the user name to use (must be always the same for a vault file!), by default: $FREEPASS_NAME")
        (@subcommand interact =>
            (about: "Launches interactive mode")
        )
    ).get_matches();

    let file_path = opt_or_env(&matches, "FILE", "FREEPASS_FILE");
    let user_name = opt_or_env(&matches, "NAME", "FREEPASS_NAME");

    sodiumoxide::init();

    // Do this early because we don't want to ask for the password when we get permission denied or something.
    // Ensure we can write! Maybe someone somewhere would want to open the vault in read-only mode...
    // But the frustration of trying to save the vault while only having read permissions would be worse.
    let file = match fs::OpenOptions::new().read(true).write(true).open(&file_path) {
        Ok(file) => Some(file),
        Err(ref err) if err.kind() == io::ErrorKind::NotFound => None,
        Err(ref err) => panic!("Could not open file {}: {}", &file_path, err),
    };

    let master_key = {
        let read_result = read_from_tty(|buf, _, tty| {
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
        }, true).unwrap();
        gen_master_key(SecStr::new(read_result), &user_name).unwrap()
    };
    let outer_key = gen_outer_key(&master_key);

    let mut vault = match file {
        Some(f) => Vault::open(&outer_key, f).unwrap(),
        None => Vault::new(),
    };

    
    {// XXX: TEST ENTRY
        let entries_key = gen_entries_key(&master_key);
        let mut twitter = Entry::new();
            twitter.fields.insert("password".to_owned(), Field::Derived {
                counter: 4, site_name: Some("twitter.com".to_owned()), usage: DerivedUsage::Password(PasswordTemplate::Maximum)
            });
            twitter.fields.insert("old_password".to_owned(), Field::Stored {
                data: SecStr::from("h0rse"), usage: StoredUsage::Password
            });
        let mut metadata = EntryMetadata::new();
        vault.put_entry(&entries_key, "twitter", &twitter, &mut metadata).unwrap();
    }

    interact_entries(&mut vault, &master_key);
}

macro_rules! interaction {
    ( { $($action_name:expr => $action_fn:expr),+ }, $data:expr, $data_fn:expr ) => {
        {
            let mut items = vec![$(">> ".to_string() + $action_name),+];
            let data_items : Vec<String> = $data.clone().map(|x| " | ".to_string() + x).collect();
            items.extend(data_items.iter().cloned());
            match pick_from_list(default_menu_cmd().as_mut(), &items[..], "Selection: ").unwrap() {
                $(ref x if *x == ">> ".to_string() + $action_name => $action_fn),+
                ref x if data_items.contains(x) => ($data_fn)(&x[3..]),
                ref x => panic!("Unknown selection: {}", x),
            }
        }
    }
}

fn interact_entries(vault: &mut Vault, master_key: &SecStr) {
    let entries_key = gen_entries_key(&master_key);
    interaction!({
        "Add new entry" => {
            println!("YO");
        }
    }, vault.entry_names(), { |x|
        println!("pick {}", x)
    })
}
