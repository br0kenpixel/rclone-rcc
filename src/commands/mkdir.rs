use rclone_crypt::{cipher::Cipher, stream::EncryptedReader};
use spinoff::{spinners, Color, Spinner};
use std::{
    fs,
    io::{stdout, Read, Write},
    path::PathBuf,
};

pub fn mkdir(dir: PathBuf, path: PathBuf, password: String, salt: Option<String>) -> i32 {
    if !dir.is_dir() {
        eprintln!("invalid directory");
        return 1;
    }

    let spinner = Spinner::new(spinners::Dots, "Creating cypher...", Color::White);
    let cipher = match Cipher::new(password, salt) {
        Ok(c) => c,
        Err(e) => {
            spinner.fail(&format!("Failed to create cipher: {e}"));
            return 1;
        }
    };
    spinner.success("Created cypher");

    let spinner = Spinner::new(spinners::Dots, "Checking paths...", Color::White);
    let encrypted_path = cipher.encrypt_path(&path).unwrap();
    let real_path = dir.join(encrypted_path);

    if real_path.is_dir() || real_path.is_file() {
        spinner.fail(&format!(
            "File or directory '{}' already exists",
            path.display()
        ));
        return 1;
    }

    spinner.success("Paths verified");
    let spinner = Spinner::new(spinners::Dots, "Creating directory...", Color::White);

    if let Err(e) = fs::create_dir(real_path) {
        spinner.fail(&format!("Failed to create directory: {e}"));
        return 1;
    }
    spinner.success("Created directory");

    0
}
