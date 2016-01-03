#[macro_use] extern crate clap;
extern crate secstr;
extern crate colorhash256;
extern crate interactor;
extern crate rusterpassword;
extern crate ansi_term;
extern crate rustc_serialize;
extern crate cbor;
extern crate freepass_core;

mod util;

use std::{fs,env,io};
use std::io::prelude::*;
use std::collections::btree_map::BTreeMap;
use rustc_serialize::base64::{ToBase64, STANDARD};
use rustc_serialize::hex::ToHex;
use secstr::SecStr;
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
        (@arg DEBUG: --debug "Enables logging of data structures for debugging (DO NOT USE ON YOUR REAL DATA)")
        (@subcommand interact =>
            (about: "Launches interactive mode")
        )
    ).get_matches();

    let file_path = opt_or_env(&matches, "FILE", "FREEPASS_FILE");
    let user_name = opt_or_env(&matches, "NAME", "FREEPASS_NAME");
    let debug = matches.is_present("DEBUG");

    freepass_core::init();

    // Do this early because we don't want to ask for the password when we get permission denied or something.
    // Ensure we can write! Maybe someone somewhere would want to open the vault in read-only mode...
    // But the frustration of trying to save the vault while only having read permissions would be worse.
    let file = match fs::OpenOptions::new().read(true).write(true).open(&file_path) {
        Ok(file) => Some(file),
        Err(ref err) if err.kind() == io::ErrorKind::NotFound => None,
        Err(ref err) => panic!("Could not open file {}: {}", &file_path, err),
    };

    let master_key = gen_master_key(util::read_password(), &user_name).unwrap();
    let outer_key = gen_outer_key(&master_key);

    let mut vault = match file {
        Some(f) => Vault::open(&outer_key, f).unwrap(),
        None => Vault::new(),
    };

    if debug {
        debug_output(&vault, "Vault");
    }

    match matches.subcommand() {
        ("interact", _) | _  => interact_entries(&mut vault, &file_path, &outer_key, &master_key, debug),
    }
}

fn opt_or_env(matches: &clap::ArgMatches, opt_name: &str, env_name: &str) -> String {
    match matches.value_of(opt_name).map(|x| x.to_owned()).or(env::var_os(env_name).and_then(|s| s.into_string().ok())) {
        Some(s) => s,
        None => panic!("Option {} or environment variable {} not found", opt_name, env_name)
    }
}

fn debug_output<T: rustc_serialize::Encodable>(data: &T, description: &str) {
    let mut e = cbor::Encoder::from_memory();
    e.encode(&[data]).unwrap();
    println!("--- CBOR debug output (http://cbor.me to decode) of {} ---\n{}\n", description, e.into_bytes().to_hex());
}

macro_rules! interaction {
    ( { $($action_name:expr => $action_fn:expr),+ }, $data:expr, $data_fn:expr ) => {
        {
            let mut items = vec![$(">> ".to_string() + $action_name),+];
            let data_items : Vec<String> = $data.clone().map(|x| " | ".to_string() + x).collect();
            items.extend(data_items.iter().cloned());
            match pick_from_list(util::menu_cmd().as_mut(), &items[..], "Selection: ").unwrap() {
                $(ref x if *x == ">> ".to_string() + $action_name => $action_fn),+
                ref x if data_items.contains(x) => ($data_fn)(&x[3..]),
                ref x => panic!("Unknown selection: {}", x),
            }
        }
    };
    ( { $($action_name:expr => $action_fn:expr),+ }) => {
        {
            let items = vec![$(">> ".to_string() + $action_name),+];
            match pick_from_list(util::menu_cmd().as_mut(), &items[..], "Selection: ").unwrap() {
                $(ref x if *x == ">> ".to_string() + $action_name => $action_fn),+
                ref x => panic!("Unknown selection: {}", x),
            }
        }
    }
}

fn interact_entries(vault: &mut Vault, file_path: &str, outer_key: &SecStr, master_key: &SecStr, debug: bool) {
    let entries_key = gen_entries_key(&master_key);
    loop {
        interaction!({
            "Quit" => {
                return ();
            },
            "Add new entry" => {
                if let Some(entry_name) = util::read_text("Entry name") {
                    interact_entry_edit(vault, file_path, outer_key, master_key, &entries_key, &entry_name, Entry::new(), EntryMetadata::new());
                }
            }
        }, vault.entry_names(), |name| {
            let (entry, meta) = vault.get_entry(&entries_key, name).unwrap();
            if debug {
                debug_output(&entry, &format!("Entry: {}", name));
            }
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
            &format!("Name:          {}", entry_name) => {},
            &format!("Last modified: {}", meta.updated_at.to_rfc2822()) => {},
            &format!("Created:       {}", meta.created_at.to_rfc2822()) => {},
            "Edit" => {
                return interact_entry_edit(vault, file_path, outer_key, master_key, entries_key, entry_name, entry, meta);
            }
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
                Output::Ed25519Keypair(usage, _, _) => match usage {
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
        &format!("  Save entry [{}]", entry_name) => {
            vault.put_entry(entries_key, entry_name, &entry, &mut meta).unwrap();
            save_vault(vault, file_path, outer_key);
            return interact_entry(vault, file_path, outer_key, master_key, entries_key, entry_name, entry, meta);
        },
        &format!("Delete entry [{}]", entry_name) => {
            interaction!({
                "Cancel" => {
                    return interact_entry_edit(vault, file_path, outer_key, master_key, entries_key, entry_name, entry, meta);
                },
                &format!("DELETE THE ENTRY '{}'!", entry_name) => {
                    vault.remove_entry(entry_name);
                    save_vault(vault, file_path, outer_key);
                    return ();
                }
            })
        },
        &format!("Rename entry [{}]", entry_name) => {
            let new_entry_name = util::read_text(&format!("New entry name [{}]", entry_name)).unwrap_or(entry_name.to_string());
            vault.remove_entry(entry_name);
            vault.put_entry(entries_key, &new_entry_name, &entry, &mut meta).unwrap();
            return interact_entry_edit(vault, file_path, outer_key, master_key, entries_key, &new_entry_name, entry, meta);
        },
        "Add field" => {
            if let Some(field_name) = util::read_text("Field name") {
                entry = interact_field_edit(vault, entry, field_name);
            }
            return interact_entry_edit(vault, file_path, outer_key, master_key, entries_key, entry_name, entry, meta);
        }
    }, entry.fields.keys(), |name: &str| {
        entry = interact_field_edit(vault, entry, name.to_string());
        return interact_entry_edit(vault, file_path, outer_key, master_key, entries_key, entry_name, entry, meta);
    });
}

fn new_derived_field(field_name: &str) -> Field {
    let fname = field_name.to_lowercase();
    if fname.contains("key") {
        Field::Derived { counter: 1, site_name: None, usage: DerivedUsage::Ed25519Key(Ed25519Usage::SSH) }
    } else {
        Field::Derived { counter: 1, site_name: None, usage: DerivedUsage::Password(PasswordTemplate::Maximum) }
    }
}

fn new_stored_field(field_name: &str) -> Field {
    let fname = field_name.to_lowercase();
    if fname.contains("name") || fname.contains("login") || fname.contains("email") {
        Field::Stored { data: SecStr::new(Vec::new()), usage: StoredUsage::Text }
    } else {
        Field::Stored { data: SecStr::new(Vec::new()), usage: StoredUsage::Password }
    }
}

fn new_field(field_name: &str) -> Field {
    let fname = field_name.to_lowercase();
    if fname.contains("name") || fname.contains("login") || fname.contains("email") {
        Field::Stored { data: SecStr::new(Vec::new()), usage: StoredUsage::Text }
    } else if fname.contains("key") {
        Field::Derived { counter: 1, site_name: None, usage: DerivedUsage::Ed25519Key(Ed25519Usage::SSH) }
    } else {
        Field::Derived { counter: 1, site_name: None, usage: DerivedUsage::Password(PasswordTemplate::Maximum) }
    }
}

fn interact_field_edit(vault: &mut Vault, mut entry: Entry, field_name: String) -> Entry {
    let mut field = entry.fields.remove(&field_name).unwrap_or_else(|| new_field(&field_name));
    let mut field_actions : BTreeMap<String, Box<Fn(Field) -> Field>> = BTreeMap::new();
    let other_type;
    match field.clone() {
        Field::Derived { counter, site_name, usage } => {
            other_type = "stored";
            field_actions.insert(format!("Counter: {}", counter), Box::new(|f| {
                if let Field::Derived { counter, site_name, usage } = f {
                    let new_counter = util::read_text(&format!("Counter [{}]", counter)).and_then(|c| c.parse::<u32>().ok()).unwrap_or(counter);
                    Field::Derived { counter: new_counter, site_name: site_name, usage: usage }
                } else { unreachable!(); }
            }));
            field_actions.insert(match site_name {
                Some(ref sn) => format!("Site name: {}", sn),
                None => format!("Site name: <same as entry name>"),
            }, Box::new(|f| {
                if let Field::Derived { counter, usage, .. } = f {
                    let new_site_name = util::read_text("Site name");
                    Field::Derived { counter: counter, site_name: new_site_name, usage: usage }
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
        Field::Stored { usage, data, .. } => {
            other_type = "derived";
            let txt = String::from_utf8(data.unsecure().to_vec()).unwrap_or("<invalid UTF-8>".to_string());
            field_actions.insert(format!("Change text [{}]", txt), Box::new(|f| {
                if let Field::Stored { usage, data, .. } = f {
                    let txt = String::from_utf8(data.unsecure().to_vec()).unwrap_or("<invalid UTF-8>".to_string());
                    let new_data = util::read_text(&format!("New text [{}]", txt)).unwrap_or(txt);
                    Field::Stored { data: SecStr::from(new_data), usage: usage }
                } else { unreachable!(); }
            }));
            field_actions.insert(format!("Usage: {:?}", usage), Box::new(|f| {
                if let Field::Stored { data, .. } = f {
                    let new_usage = interaction!({
                        "Password"            => { StoredUsage::Password },
                        "Text"                => { StoredUsage::Text }
                    });
                    Field::Stored { data: data, usage: new_usage }
                } else { unreachable!(); }
            }));
        }
    };
    interaction!({
        "Go back" => {
            entry.fields.insert(field_name, field);
            return entry;
        },
        &format!("Delete field [{}]", field_name) => {
            interaction!({
                "Cancel" => {
                    return interact_field_edit(vault, entry, field_name);
                },
                &format!("DELETE THE FIELD '{}'!", field_name) => {
                    entry.fields.remove(&field_name);
                    return entry;
                }
            })
        },
        &format!("Rename field [{}]", field_name) => {
            let new_field_name = util::read_text(&format!("New field name [{}]", field_name)).unwrap_or(field_name.to_string());
            entry.fields.insert(new_field_name.clone(), field);
            return interact_field_edit(vault, entry, new_field_name);
        },
        &format!("Change type to {}", other_type) => {
            entry.fields.remove(&field_name);
            entry.fields.insert(field_name.clone(), match field {
                Field::Derived { .. } => new_stored_field(&field_name),
                Field::Stored { .. } => new_derived_field(&field_name),
            });
            return interact_field_edit(vault, entry, field_name);
        }
    }, field_actions.keys(), |key| {
        field = field_actions.get(key).unwrap()(field);
        entry.fields.insert(field_name.clone(), field);
        return interact_field_edit(vault, entry, field_name);
    })
}

fn save_vault(vault: &mut Vault, file_path: &str, outer_key: &SecStr) {
    // Atomic save!
    vault.save(outer_key, fs::File::create(format!("{}.tmp", file_path)).unwrap()).unwrap();
    fs::rename(format!("{}.tmp", file_path), file_path).unwrap();
}
