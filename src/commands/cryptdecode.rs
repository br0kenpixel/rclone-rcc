use crate::macros::create_cipher;
use rclone_crypt::cipher::Cipher;
use std::path::PathBuf;

pub fn cryptdecode(
    filename: PathBuf,
    password: String,
    salt: Option<String>,
    reverse: bool,
) -> i32 {
    let salt = salt.as_deref();
    create_cipher!(cipher, &password, salt);

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
