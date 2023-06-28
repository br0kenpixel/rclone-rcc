use crate::macros::create_cipher;
use chrono::{DateTime, Utc};
use rclone_crypt::cipher::Cipher;
use spinoff::{spinners, Color, Spinner};
use std::{fs, path::PathBuf};

pub fn lsd(dir: PathBuf, password: String, salt: Option<String>) -> i32 {
    if !dir.is_dir() {
        eprintln!("invalid directory");
        return 1;
    }
    let salt = salt.as_deref();

    create_cipher!(cipher, &password, salt);

    let spinner = Spinner::new(spinners::Dots, "Decrypting file names...", Color::White);

    let files_iter = fs::read_dir(&dir)
        .unwrap()
        .map(Result::unwrap)
        .filter(|entry| entry.file_type().unwrap().is_dir())
        .map(|entry| entry.file_name().into_string().unwrap());
    let mut folders = Vec::new();

    for entry in files_iter {
        let decrypted_name = match cipher.decrypt_file_name(&entry) {
            Ok(name) => name,
            Err(e) => {
                spinner.fail(&format!("Failed to decrypt \"{}\": {}", entry, e));
                return 1;
            }
        };

        folders.push((dir.join(entry), decrypted_name));
    }

    spinner.success("Listing");

    for entry in folders {
        let real_path = entry.0;
        let folder = entry.1;

        let modtime = fs::metadata(real_path).unwrap().modified().unwrap();
        let modtime: DateTime<Utc> = modtime.into();
        println!(
            "{:>12} {} {} {:>9} {folder}",
            -1,
            modtime.format("%Y-%m-%d"),
            modtime.format("%H:%M:%S"),
            -1
        );
    }

    0
}
