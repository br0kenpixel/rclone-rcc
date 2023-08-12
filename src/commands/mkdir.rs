use crate::macros::create_cipher;
use rclone_crypt::cipher::Cipher;
use std::{fs, path::PathBuf};

pub fn mkdir(dir: PathBuf, path: PathBuf, password: String, salt: Option<String>) -> i32 {
    if !dir.is_dir() {
        eprintln!("Invalid directory");
        return 1;
    }
    let salt = salt.as_deref();
    create_cipher!(cipher, &password, salt);

    let encrypted_path = cipher.encrypt_path(&path).unwrap();
    let real_path = dir.join(encrypted_path);

    if real_path.is_dir() || real_path.is_file() {
        eprintln!("File or directory '{}' already exists", path.display());
        return 1;
    }

    if let Err(e) = fs::create_dir(real_path) {
        eprintln!("Failed to create directory: {e}");
        return 1;
    }

    0
}
