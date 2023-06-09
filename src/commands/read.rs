use crate::macros::create_cipher;
use rclone_crypt::{cipher::Cipher, stream::EncryptedReader};
use spinoff::{spinners, Color, Spinner};
use std::{
    fs,
    io::{stdout, Read, Write},
    path::PathBuf,
};

pub fn read(
    dir: PathBuf,
    file: PathBuf,
    password: String,
    salt: Option<String>,
    offset: usize,
    amount: usize,
) -> i32 {
    if !dir.is_dir() {
        eprintln!("invalid directory");
        return 1;
    }
    let salt = salt.as_deref();

    create_cipher!(cipher, &password, salt);

    let spinner = Spinner::new(spinners::Dots, "Decrypting...", Color::White);
    let encrypted_path = cipher.encrypt_path(&file).unwrap();
    let real_path = dir.join(encrypted_path);

    if !real_path.is_file() {
        spinner.fail(&format!("File '{}' does not exist", file.display()));
        return 1;
    }

    let file_stream = fs::OpenOptions::new().read(true).open(real_path).unwrap();
    let mut reader = EncryptedReader::new(file_stream, &password, salt).unwrap();

    let mut buf = Vec::new();
    reader.read_to_end(&mut buf).unwrap();
    spinner.success("Decrypted successfully");

    let spinner = Spinner::new(spinners::Dots, "Reading...", Color::White);

    let part = match buf.get(offset..(offset + amount)) {
        Some(part) => part,
        None => {
            spinner.fail("Indexes are outside bounds");
            return 1;
        }
    };
    spinner.success("Read");

    stdout().write_all(part).unwrap();
    0
}
