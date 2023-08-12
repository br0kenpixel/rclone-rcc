use crate::macros::create_cipher;
use rclone_crypt::{
    cipher::Cipher,
    stream::{EncryptedReader, EncryptedWriter},
};
use std::{
    fs,
    io::{Read, Write},
    path::PathBuf,
};

pub fn cp(
    dir: PathBuf,
    file: PathBuf,
    dest: PathBuf,
    password: String,
    salt: Option<String>,
    reverse: bool,
    move_: bool,
) -> i32 {
    /*
        By default this command should copy a file from the encrypted directory to a location outside of it.
        If `reverse` is `true`, the file should be copied from outside the encrypted directory into it.
    */

    let salt = salt.as_deref();
    create_cipher!(cipher, &password, salt);

    if !reverse {
        // Check if we can access the encrypted directory and the source file exists in it.

        if !dir.is_dir() {
            eprintln!("invalid directory");
            return 1;
        }

        let encrypted_path = cipher.encrypt_path(&file).unwrap();
        let real_path = dir.join(encrypted_path);

        if !real_path.is_file() {
            eprintln!("File '{}' does not exist", file.display());
            return 1;
        }

        copy_from_encrypted_dir(dir, file, dest, cipher, move_)
    } else {
        // Check if the encrypted directory exists as well as the source file.

        let src = dest;
        let dest = file;

        if !dir.is_dir() {
            eprintln!("Invalid directory");
            return 1;
        }

        let encrypted_path = cipher.encrypt_path(&dest).unwrap();
        let real_path = dir.join(encrypted_path);

        if real_path.is_file() {
            eprintln!("File '{}' alredy exists, not overwriting", dest.display());
            return 1;
        }

        copy_into_encrypted_dir(dir, dest, src, cipher, move_)
    }
}

/// Copy a file from the encrypted directory outside of it.
fn copy_from_encrypted_dir(
    dir: PathBuf,
    file: PathBuf,
    dest: PathBuf,
    cipher: Cipher,
    move_: bool,
) -> i32 {
    let encrypted_path = cipher.encrypt_path(&file).unwrap();
    let real_path = dir.join(encrypted_path);

    {
        let src = fs::OpenOptions::new()
            .read(true)
            .open(real_path.clone())
            .unwrap();
        let mut dest = fs::OpenOptions::new()
            .create(true)
            .write(true)
            .open(dest)
            .unwrap();
        let mut reader = EncryptedReader::new_with_cipher(src, cipher).unwrap();

        let mut buf = Vec::new();
        reader.read_to_end(&mut buf).unwrap();
        dest.write_all(&buf).unwrap();
    }

    if move_ {
        let result = delete_file(real_path);
        if result != 0 {
            return result;
        }
    }

    0
}

/// Copy a file into the encrypted directory inside of it.
fn copy_into_encrypted_dir(
    dir: PathBuf,
    dest: PathBuf,
    src: PathBuf,
    cipher: Cipher,
    move_: bool,
) -> i32 {
    let encrypted_path = cipher.encrypt_path(&dest).unwrap();
    let real_path = dir.join(encrypted_path);

    {
        let mut src = fs::OpenOptions::new().read(true).open(src).unwrap();
        let dest = fs::OpenOptions::new()
            .create(true)
            .write(true)
            .open(real_path.clone())
            .unwrap();
        let mut writer = EncryptedWriter::new_with_cipher(dest, cipher).unwrap();

        let mut buf = Vec::new();
        src.read_to_end(&mut buf).unwrap();
        writer.write_all(&buf).unwrap();
    }

    if move_ {
        let result = delete_file(real_path);
        if result != 0 {
            return result;
        }
    }

    0
}

fn delete_file(path: PathBuf) -> i32 {
    if let Err(e) = fs::remove_file(path) {
        eprintln!("Failed to move file: {e}");
        return 1;
    }

    0
}
