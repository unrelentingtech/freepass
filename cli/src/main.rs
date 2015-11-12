#[macro_use] extern crate clap;
extern crate secstr;
extern crate colorhash256;
extern crate interactor;
extern crate rusterpassword;
extern crate sodiumoxide;
extern crate ansi_term;
extern crate rustc_serialize;
extern crate freepass_core;

use std::{fs,env,io};
use std::io::prelude::*;
use std::process::{Command, Stdio};
use std::collections::btree_map::BTreeMap;
use rustc_serialize::base64::{ToBase64, STANDARD};
use rustc_serialize::hex::ToHex;
use ansi_term::Colour::Fixed;
use ansi_term::ANSIStrings;
use secstr::*;
use interactor::*;
use rusterpassword::*;
use freepass_core::data::*;
use freepass_core::output::*;

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
        let password = match env::var_os("FREEPASS_ASKPASS").or(env::var_os("ASKPASS")) {
            Some(s) => get_password_askpass(Command::new(s)),
            None => get_password_console(),
        };
        gen_master_key(password, &user_name).unwrap()
    };
    let outer_key = gen_outer_key(&master_key);

    let mut vault = match file {
        Some(f) => Vault::open(&outer_key, f).unwrap(),
        None => Vault::new(),
    };

    interact_entries(&mut vault, &file_path, &outer_key, &master_key);
}

fn opt_or_env(matches: &clap::ArgMatches, opt_name: &str, env_name: &str) -> String {
    match matches.value_of(opt_name).map(|x| x.to_owned()).or(env::var_os(env_name).and_then(|s| s.into_string().ok())) {
        Some(s) => s,
        None => panic!("Option {} or environment variable {} not found", opt_name, env_name)
    }
}

fn get_password_console() -> SecStr {
    SecStr::new(read_from_tty(|buf, b, tty| {
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

fn get_password_askpass(mut command: Command) -> SecStr {
    let process = command.stdout(Stdio::piped()).spawn().unwrap();
    let mut result = Vec::new();
    let mut reader = io::BufReader::new(process.stdout.unwrap());
    let size = reader.read_until(b'\n', &mut result).unwrap();
    result.truncate(size - 1);
    SecStr::new(result)
}

pub fn menu_cmd() -> Option<Command> {
    env::var_os("FREEPASS_MENU").or(env::var_os("MENU")).map(|s| Command::new(s))
}

macro_rules! interaction {
    ( { $($action_name:expr => $action_fn:expr),+ }, $data:expr, $data_fn:expr ) => {
        {
            let mut items = vec![$(">> ".to_string() + $action_name),+];
            let data_items : Vec<String> = $data.clone().map(|x| " | ".to_string() + x).collect();
            items.extend(data_items.iter().cloned());
            match pick_from_list(menu_cmd().as_mut(), &items[..], "Selection: ").unwrap() {
                $(ref x if *x == ">> ".to_string() + $action_name => $action_fn),+
                ref x if data_items.contains(x) => ($data_fn)(&x[3..]),
                ref x => panic!("Unknown selection: {}", x),
            }
        }
    };
    ( { $($action_name:expr => $action_fn:expr),+ }) => {
        {
            let items = vec![$(">> ".to_string() + $action_name),+];
            match pick_from_list(menu_cmd().as_mut(), &items[..], "Selection: ").unwrap() {
                $(ref x if *x == ">> ".to_string() + $action_name => $action_fn),+
                ref x => panic!("Unknown selection: {}", x),
            }
        }
    }
}

fn interact_entries(vault: &mut Vault, file_path: &str, outer_key: &SecStr, master_key: &SecStr) {
    let entries_key = gen_entries_key(&master_key);
    loop {
        interaction!({
            "Quit" => {
                return ();
            },
            "Add new entry" => {
                interact_entry_edit(vault, file_path, outer_key, master_key, &entries_key, &read_text("Entry name"), Entry::new(), EntryMetadata::new());
            }
        }, vault.entry_names(), |name| {
            let (entry, meta) = vault.get_entry(&entries_key, name).unwrap();
            interact_entry(vault, file_path, outer_key, master_key, &entries_key, name, entry, meta);
        });
    }
}

fn interact_entry(vault: &mut Vault, file_path: &str, outer_key: &SecStr, master_key: &SecStr, entries_key: &SecStr, entry_name: &str, entry: Entry, meta: EntryMetadata) {
    loop {
        interaction!({
            "Go back" => {
                return ();
            },
            "Edit" => {
                return interact_entry_edit(vault, file_path, outer_key, master_key, entries_key, entry_name, entry, meta);
            },
            &format!("Last modified: {}", meta.updated_at.to_rfc2822()) => {},
            &format!("Created:       {}", meta.created_at.to_rfc2822()) => {}
        }, entry.fields.keys(), |name: &str| {
            let output = process_output(entry_name, master_key, entry.fields.get(name).unwrap()).unwrap();
            match output {
                Output::PrivateText(s) => println!("{}", String::from_utf8(Vec::from(s.unsecure())).unwrap()),
                Output::OpenText(s) => println!("{}", s),
                Output::PrivateBinary(s) => {
                    interaction!({
                        "Go back" => {},
                        "Print as Base64" => { println!("{}", s.unsecure().to_base64(STANDARD)) },
                        "Print as hex" => { println!("{}", s.unsecure().to_hex()) }
                    })
                },
                Output::Ed25519Keypair(usage, ref pubkey, ref seckey) => match usage {
                    Ed25519Usage::SSH => {
                        interaction!({
                            "Go back" => {},
                            "Print public key" => { println!("{}", ssh_public_key_output(&output, entry_name).unwrap()) },
                            "Add private key to ssh-agent" => {
                                ssh_agent_send_message(ssh_private_key_agent_message(&output, entry_name).unwrap()).unwrap()
                            }
                        })
                    }
                    _ => panic!("Unsupported key usage"),
                }
            }
        });
    }
}

fn interact_entry_edit(vault: &mut Vault, file_path: &str, outer_key: &SecStr, master_key: &SecStr, entries_key: &SecStr, entry_name: &str, mut entry: Entry, mut meta: EntryMetadata) {
    interaction!({
        "Save" => {
            return interact_entry(vault, file_path, outer_key, master_key, entries_key, entry_name, entry, meta);
        },
        "Add field" => {
            entry = interact_field_edit(vault, entry, read_text("Field name"));
            save_field(vault, file_path, outer_key, entries_key, entry_name, &entry, &mut meta);
            return interact_entry_edit(vault, file_path, outer_key, master_key, entries_key, entry_name, entry, meta);
        }
    }, entry.fields.keys(), |name: &str| {
        entry = interact_field_edit(vault, entry, name.to_string());
        save_field(vault, file_path, outer_key, entries_key, entry_name, &entry, &mut meta);
        return interact_entry_edit(vault, file_path, outer_key, master_key, entries_key, entry_name, entry, meta);
    });
}

fn interact_field_edit(vault: &mut Vault, mut entry: Entry, field_name: String) -> Entry {
    let mut field = entry.fields.remove(&field_name).unwrap_or(
        Field::Derived { counter: 0, site_name: None, usage: DerivedUsage::Password(PasswordTemplate::Maximum) });
    let mut field_actions : BTreeMap<String, Box<Fn(Field) -> Field>> = BTreeMap::new();
    match field.clone() {
        Field::Derived { counter, site_name, usage } => {
            field_actions.insert(format!("Counter: {}", counter), Box::new(|f| {
                if let Field::Derived { counter, site_name, usage } = f {
                    let new_counter = read_text(&format!("Counter [{}]", counter)).parse::<u32>().ok().unwrap_or(counter);
                    Field::Derived { counter: new_counter, site_name: site_name, usage: usage }
                } else { unreachable!(); }
            }));
            field_actions.insert(match site_name {
                Some(ref sn) => format!("Site name: {}", sn),
                None => format!("Site name: <same as entry name>"),
            }, Box::new(|f| {
                if let Field::Derived { counter, usage, .. } = f {
                    let new_site_name = read_text("Site name");
                    Field::Derived { counter: counter, site_name: if new_site_name.len() == 0 { None } else { Some(new_site_name) }, usage: usage }
                } else { unreachable!(); }
            }));
            field_actions.insert(format!("Usage: {:?}", usage), Box::new(|f| {
                if let Field::Derived { counter, site_name, .. } = f {
                    let new_usage = interaction!({
                        "Password(Maximum)"   => { DerivedUsage::Password(PasswordTemplate::Maximum) },
                        "Password(Long)"      => { DerivedUsage::Password(PasswordTemplate::Long) },
                        "Password(Medium)"    => { DerivedUsage::Password(PasswordTemplate::Medium) },
                        "Password(Short)"     => { DerivedUsage::Password(PasswordTemplate::Short) },
                        "Password(Basic)"     => { DerivedUsage::Password(PasswordTemplate::Basic) },
                        "Password(Pin)"       => { DerivedUsage::Password(PasswordTemplate::Pin) },
                        "Ed25519Key(SSH)"     => { DerivedUsage::Ed25519Key(Ed25519Usage::SSH) },
                        "Ed25519Key(Signify)" => { DerivedUsage::Ed25519Key(Ed25519Usage::Signify) },
                        "Ed25519Key(SQRL)"    => { DerivedUsage::Ed25519Key(Ed25519Usage::SQRL) },
                        "RawKey"              => { DerivedUsage::RawKey }
                    });
                    Field::Derived { counter: counter, site_name: site_name, usage: new_usage }
                } else { unreachable!(); }
            }));
        },
        Field::Stored { .. } => {
            // TODO
        }
    };
    interaction!({
        "Go back" => {
            entry.fields.insert(field_name, field);
            return entry;
        },
        &format!("Rename field [{}]", field_name) => {
            let mut new_field_name = read_text(&format!("New field name [{}]", field_name));
            if new_field_name.len() == 0 {
                new_field_name = field_name.to_string();
            }
            entry.fields.insert(new_field_name.clone(), field);
            return interact_field_edit(vault, entry, new_field_name);
        }
    }, field_actions.keys(), |key| {
        field = field_actions.get(key).unwrap()(field);
        entry.fields.insert(field_name.clone(), field);
        return interact_field_edit(vault, entry, field_name);
    })
}

fn save_field(vault: &mut Vault, file_path: &str, outer_key: &SecStr, entries_key: &SecStr, entry_name: &str, entry: &Entry, meta: &mut EntryMetadata) {
    vault.put_entry(entries_key, entry_name, entry, meta).unwrap();
    // Atomic save!
    vault.save(outer_key, fs::File::create(format!("{}.tmp", file_path)).unwrap()).unwrap();
    fs::rename(format!("{}.tmp", file_path), file_path).unwrap();
}

fn read_text(prompt: &str) -> String {
    let mut tty = fs::OpenOptions::new().read(true).write(true).open("/dev/tty").unwrap();
    tty.write(&format!("\r{}: ", prompt).into_bytes()).unwrap();
    let mut reader = io::BufReader::new(tty);
    let mut input = String::new();
    reader.read_line(&mut input).unwrap();
    input.replace("\n", "")
}
