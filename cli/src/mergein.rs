use freepass_core::merge::*;
use freepass_core::vault::{Vault, WritableVault};
use openfile::OpenFile;
use util;

pub fn merge_in(into_open_file: &mut OpenFile, from_open_file: &OpenFile) {
    let log = merge_vaults(&mut into_open_file.vault, &from_open_file.vault);
    for lentry in &log {
        match *lentry {
            MergeLogEntry::Added(ref entry_name) => println!("Added: {}", entry_name),
            MergeLogEntry::IsNewer(_) => (),
            MergeLogEntry::IsOlder(ref entry_name) => println!("Not updated in the second file: {}", entry_name),
            MergeLogEntry::WeirdError(ref entry_name) => println!("ERROR! Couldn't add: {}", entry_name),
        }
    }
    // Handling all IsNewers together for better output
    for lentry in &log {
        if let MergeLogEntry::IsNewer(ref entry_name) = *lentry {
            if util::read_yesno(&format!("Update entry '{}'?", entry_name)) {
                if let Ok((from_entry, from_entry_meta)) = from_open_file.vault.get_entry(&entry_name) {
                    if let Ok(_) = into_open_file.vault.put_entry(&entry_name, &from_entry, &mut from_entry_meta.clone()) {
                        println!("Added: {}", entry_name)
                    } else {
                        println!("ERROR! Couldn't add: {}", entry_name)
                    }
                } else {
                    println!("ERROR! Couldn't add: {}", entry_name)
                }
            }
        }
    }
    into_open_file.save();
}
