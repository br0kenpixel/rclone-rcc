use rclone_crypt::cipher::Cipher;
use spinoff::{spinners, Color, Spinner};
use std::{fs, path::PathBuf};

pub fn ls(dir: PathBuf, password: String, salt: Option<String>) -> i32 {
    if !dir.is_dir() {
        eprintln!("invalid directory");
        return 1;
    }

    let spinner = Spinner::new(spinners::Dots, "Creating cipher...", Color::White);
    let cipher = match Cipher::new(password, salt) {
        Ok(c) => c,
        Err(e) => {
            spinner.fail(&format!("Failed to create cipher: {e}"));
            return 1;
        }
    };
    spinner.success("Created cipher");

    let spinner = Spinner::new(spinners::Dots, "Decrypting file names...", Color::White);

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
                spinner.fail(&format!("Failed to decrypt \"{}\": {}", entry, e));
                return 1;
            }
        };

        files.push((dir.join(entry), decrypted_name));
    }

    spinner.success("Listing");

    for file in files {
        let real_path = file.0;
        let file = file.1;
        let size = fs::metadata(real_path).unwrap().len();

        println!("{:>9} {file}", size);
    }

    0
}
