use vault::{Vault, WritableVault};

pub enum MergeLogEntry {
    Added(String),
    IsNewer(String),
    IsOlder(String),
    WeirdError(String),
}

pub fn merge_vaults<I: Vault + WritableVault + ?Sized, F: Vault + ?Sized>(into_vault: &mut I, from_vault: &F) -> Vec<MergeLogEntry> {
    let mut results = Vec::with_capacity(from_vault.len());
    for entry_name in from_vault.entry_names() {
        if let Ok((from_entry, from_entry_meta)) = from_vault.get_entry(entry_name) {
            // After changing the into_vault type to be generic, the borrow checker started
            // complaining about multiple borrows o_0 So we use a raw pointer here.
            // Probably this is related: https://github.com/rust-lang/rfcs/issues/811
            if let Some(_) = unsafe { (*(into_vault as *const I)).entry_names().find(|&n| n == entry_name) } {
                if let Ok((_, into_entry_meta)) = into_vault.get_entry(entry_name) {
                    if from_entry_meta.updated_at > into_entry_meta.updated_at {
                        results.push(MergeLogEntry::IsNewer(entry_name.to_string()));
                    } else {
                        results.push(MergeLogEntry::IsOlder(entry_name.to_string()));
                    }
                } else {
                    results.push(MergeLogEntry::WeirdError(entry_name.to_string()));
                }
            } else {
                if let Ok(_) = into_vault.put_entry(entry_name, &from_entry, &mut from_entry_meta.clone()) {
                    results.push(MergeLogEntry::Added(entry_name.to_string()));
                } else {
                    results.push(MergeLogEntry::WeirdError(entry_name.to_string()));
                }
            }
        } else {
            results.push(MergeLogEntry::WeirdError(entry_name.to_string()));
        }
    }
    results
}
