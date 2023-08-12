use crate::macros::create_cipher;
use rclone_crypt::cipher::Cipher;
use std::{fs, path::PathBuf};

pub fn ls(dir: PathBuf, password: String, salt: Option<String>) -> i32 {
    if !dir.is_dir() {
        eprintln!("invalid directory");
        return 1;
    }
    let salt = salt.as_deref();
    create_cipher!(cipher, &password, salt);

    let files_iter = fs::read_dir(&dir)
        .unwrap()
        .map(Result::unwrap)
        .filter(|entry| entry.file_type().unwrap().is_file())
        .map(|entry| entry.file_name().into_string().unwrap());
    let mut files = Vec::new();

    for entry in files_iter {
        let decrypted_name = match cipher.decrypt_file_name(&entry) {
            Ok(name) => name,
            Err(e) => {
                eprintln!("Failed to decrypt \"{}\": {}", entry, e);
                return 1;
            }
        };

        files.push((dir.join(entry), decrypted_name));
    }

    for file in files {
        let real_path = file.0;
        let file = file.1;
        let size = fs::metadata(real_path).unwrap().len();

        println!("{:>9} {file}", size);
    }

    0
}
