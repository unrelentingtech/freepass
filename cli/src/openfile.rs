use std::{fs,io};
use secstr::SecStr;
use rusterpassword::gen_master_key;
use freepass_core::encvault::*;

pub struct OpenFile {
    pub vault: DecryptedVault,
    pub master_key: SecStr,
    pub file_path: String
}

impl OpenFile {
    pub fn open(file_path: String, user_name: &str, password: SecStr, need_write: bool) -> OpenFile {
        let file = match fs::OpenOptions::new().read(true).write(need_write).open(&file_path) {
            Ok(file) => Some(file),
            Err(ref err) if err.kind() == io::ErrorKind::NotFound => None,
            Err(ref err) => panic!("Could not open file {}: {}", &file_path, err),
        };
        let master_key = gen_master_key(password, user_name).unwrap();
        OpenFile {
            vault: match file {
                Some(f) => DecryptedVault::open(gen_entries_key(&master_key), gen_outer_key(&master_key), f).expect("Couldn't read/decrypt freepass vault"),
                None => DecryptedVault::new(gen_entries_key(&master_key), gen_outer_key(&master_key)),
            },
            master_key: master_key,
            file_path: file_path,
        }
    }

    pub fn save(self: &mut OpenFile) {
        // Atomic save!
        self.vault.save(fs::File::create(format!("{}.tmp", &self.file_path)).unwrap()).unwrap();
        fs::rename(format!("{}.tmp", &self.file_path), &self.file_path).unwrap();
    }
}
