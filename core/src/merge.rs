use secstr::SecStr;
use data::*;

pub enum MergeLogEntry {
    Added(String),
    IsNewer(String),
    IsOlder(String),
    WeirdError(String),
}

pub fn merge_vaults(into_vault: &mut Vault, into_entries_key: &SecStr, from_vault: &Vault, from_entries_key: &SecStr) -> Vec<MergeLogEntry> {
    let mut results = Vec::with_capacity(from_vault.entry_names().len());
    for entry_name in from_vault.entry_names() {
        if let Ok((from_entry, from_entry_meta)) = from_vault.get_entry(from_entries_key, entry_name) {
            if let Some(_) = into_vault.entry_names().find(|&n| n == entry_name) {
                if let Ok((_, into_entry_meta)) = into_vault.get_entry(into_entries_key, entry_name) {
                    if from_entry_meta.updated_at > into_entry_meta.updated_at {
                        results.push(MergeLogEntry::IsNewer(entry_name.to_string()));
                    } else {
                        results.push(MergeLogEntry::IsOlder(entry_name.to_string()));
                    }
                } else {
                    results.push(MergeLogEntry::WeirdError(entry_name.to_string()));
                }
            } else {
                if let Ok(_) = into_vault.put_entry(into_entries_key, entry_name, &from_entry, &mut from_entry_meta.clone()) {
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
