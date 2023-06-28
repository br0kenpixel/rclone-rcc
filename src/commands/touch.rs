use crate::macros::create_cipher;
use rclone_crypt::cipher::Cipher;
use spinoff::{spinners, Color, Spinner};
use std::{fs, path::PathBuf};

pub fn touch(dir: PathBuf, file: PathBuf, password: String, salt: Option<String>) -> i32 {
    if !dir.is_dir() {
        eprintln!("invalid directory");
        return 1;
    }
    let salt = salt.as_deref();

    create_cipher!(cipher, &password, salt);

    let spinner = Spinner::new(spinners::Dots, "Checking paths...", Color::White);
    let encrypted_path = cipher.encrypt_path(&file).unwrap();
    let real_path = dir.join(encrypted_path);

    if !real_path.is_file() {
        spinner.fail(&format!("File '{}' does not exist", file.display()));
        return 1;
    }
    spinner.success("Paths verified");

    let spinner = Spinner::new(spinners::Dots, "Touching...", Color::White);

    if let Err(e) = fs::OpenOptions::new()
        .create(true)
        .write(true)
        .open(real_path)
    {
        spinner.fail(&format!("Failed to open file: {e}"));
        return 1;
    }

    spinner.success("Touched");
    0
}
