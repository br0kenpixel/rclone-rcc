use rclone_crypt::{cipher::Cipher, stream::EncryptedReader};
use spinoff::{spinners, Color, Spinner};
use std::{fs, io::Read, path::PathBuf};

pub fn tail(dir: PathBuf, file: PathBuf, password: String, salt: Option<String>, n: usize) -> i32 {
    let spinner = Spinner::new(spinners::Dots, "Creating cypher...", Color::White);
    let cipher = match Cipher::new(password, salt) {
        Ok(c) => c,
        Err(e) => {
            spinner.fail(&format!("Failed to create cipher: {e}"));
            return 1;
        }
    };
    spinner.success("Created cypher");

    if !dir.is_dir() {
        eprintln!("invalid directory");
        return 1;
    }

    let spinner = Spinner::new(spinners::Dots, "Checking paths...", Color::White);
    let encrypted_path = cipher.encrypt_path(&file).unwrap();
    let real_path = dir.join(encrypted_path);

    if !real_path.is_file() {
        spinner.fail(&format!("File '{}' does not exist", file.display()));
        return 1;
    }
    spinner.success("Paths verified");

    let src = fs::OpenOptions::new().read(true).open(real_path).unwrap();
    let mut reader = EncryptedReader::new_with_cipher(src, cipher).unwrap();

    let spinner = Spinner::new(spinners::Dots, "Reading...", Color::White);
    let mut content = String::new();
    if let Err(e) = reader.read_to_string(&mut content) {
        spinner.fail(&format!("Failed to read file as UTF-8 text: {e}"));
        return 1;
    }
    spinner.success("Done");

    for line in content.split('\n').rev().take(n) {
        println!("{line}");
    }

    0
}
