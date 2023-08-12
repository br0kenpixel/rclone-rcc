use crate::macros::create_cipher;
use rclone_crypt::cipher::Cipher;
use std::{fs, path::PathBuf};

pub fn rm(dir: PathBuf, file: PathBuf, password: String, salt: Option<String>) -> i32 {
    if !dir.is_dir() {
        eprintln!("Invalid directory");
        return 1;
    }
    let salt = salt.as_deref();
    create_cipher!(cipher, &password, salt);

    let encrypted_path = cipher.encrypt_path(&file).unwrap();
    let real_path = dir.join(encrypted_path);

    if !real_path.is_file() {
        eprintln!("File '{}' does not exist", file.display());
        return 1;
    }

    if let Err(e) = fs::remove_file(real_path) {
        eprintln!("Failed to remove file: {e}");
        return 1;
    }

    0
}
