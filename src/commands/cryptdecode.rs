use rclone_crypt::cipher::Cipher;
use spinoff::{spinners, Color, Spinner};
use std::path::PathBuf;

pub fn cryptdecode(
    filename: PathBuf,
    password: String,
    salt: Option<String>,
    reverse: bool,
) -> i32 {
    let spinner = Spinner::new(spinners::Dots, "Creating cipher...", Color::White);
    let cipher = match Cipher::new(password, salt) {
        Ok(c) => c,
        Err(e) => {
            spinner.fail(&format!("Failed to create cipher: {e}"));
            return 1;
        }
    };
    spinner.success("Created cipher");

    let (result, operation) = if reverse {
        (cipher.encrypt_path(&filename), "encrypt")
    } else {
        (cipher.decrypt_path(&filename), "decrypt")
    };

    match result {
        Ok(result) => {
            println!("{}\t{}", filename.display(), result.display());
            0
        }
        Err(_) => {
            eprintln!("{}\tFailed to {}", filename.display(), operation);
            1
        }
    }
}
