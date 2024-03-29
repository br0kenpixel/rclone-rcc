use crate::macros::create_cipher;
use rclone_crypt::{cipher::Cipher, stream::EncryptedReader};
use std::{fs, io::Read, path::PathBuf};

pub fn tail(dir: PathBuf, file: PathBuf, password: String, salt: Option<String>, n: usize) -> i32 {
    let salt = salt.as_deref();
    create_cipher!(cipher, &password, salt);

    if !dir.is_dir() {
        eprintln!("Invalid directory");
        return 1;
    }

    let encrypted_path = cipher.encrypt_path(&file).unwrap();
    let real_path = dir.join(encrypted_path);

    if !real_path.is_file() {
        eprintln!("File '{}' does not exist", file.display());
        return 1;
    }

    let src = fs::OpenOptions::new().read(true).open(real_path).unwrap();
    let mut reader = EncryptedReader::new_with_cipher(src, cipher).unwrap();
    let mut content = String::new();

    if let Err(e) = reader.read_to_string(&mut content) {
        eprintln!("Failed to read file as UTF-8 text: {e}");
        return 1;
    }

    for line in content.split('\n').rev().take(n) {
        println!("{line}");
    }

    0
}
