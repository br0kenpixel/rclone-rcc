use rclone_crypt::{cipher::Cipher, stream::EncryptedReader};
use spinoff::{spinners, Color, Spinner};
use std::{
    fs,
    io::{stdout, Read, Write},
    path::PathBuf,
};

pub fn cat(dir: PathBuf, file: PathBuf, password: String, salt: Option<String>) -> i32 {
    if !dir.is_dir() {
        eprintln!("invalid directory");
        return 1;
    }

    let spinner = Spinner::new(spinners::Dots, "Creating cipher...", Color::White);
    let cipher = match Cipher::new(password.clone(), salt.clone()) {
        Ok(c) => c,
        Err(e) => {
            spinner.fail(&format!("Failed to create cipher: {e}"));
            return 1;
        }
    };
    spinner.success("Created cipher");

    let spinner = Spinner::new(spinners::Dots, "Decrypting...", Color::White);
    let encrypted_path = cipher.encrypt_path(&file).unwrap();
    let real_path = dir.join(encrypted_path);

    if !real_path.is_file() {
        spinner.fail(&format!("File '{}' does not exist", file.display()));
        return 1;
    }

    let file_stream = fs::OpenOptions::new().read(true).open(real_path).unwrap();
    let mut reader = EncryptedReader::new(file_stream, password, salt).unwrap();

    let mut buf = Vec::new();
    reader.read_to_end(&mut buf).unwrap();
    spinner.success("Decrypted successfully");

    stdout().write_all(&buf).unwrap();
    0
}
