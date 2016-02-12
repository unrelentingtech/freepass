use freepass_core::merge::*;
use freepass_core::vault::{Vault, WritableVault};
use util;

pub fn merge_in<I: ?Sized, F: ?Sized>(into_vault: &mut I, from_vault: &F)
where I: Vault + WritableVault, F: Vault {
    let log = merge_vaults(into_vault, from_vault);
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
                if let Ok((from_entry, from_entry_meta)) = from_vault.get_entry(&entry_name) {
                    if let Ok(_) = into_vault.put_entry(&entry_name, &from_entry, &mut from_entry_meta.clone()) {
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
}
