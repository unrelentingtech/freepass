use std::{fs,io};
use secstr::SecStr;
use rusterpassword::gen_master_key;
use freepass_core::data::*;

pub struct OpenFile {
    pub vault: Vault,
    pub master_key: SecStr,
    pub outer_key: SecStr,
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
        let outer_key = gen_outer_key(&master_key);
        OpenFile {
            vault: match file {
                Some(f) => Vault::open(&outer_key, f).unwrap(),
                None => Vault::new(),
            },
            master_key: master_key,
            outer_key: outer_key,
            file_path: file_path,
        }
    }

    pub fn save(self: &mut OpenFile) {
        // Atomic save!
        self.vault.save(&self.outer_key, fs::File::create(format!("{}.tmp", &self.file_path)).unwrap()).unwrap();
        fs::rename(format!("{}.tmp", &self.file_path), &self.file_path).unwrap();
    }
}
