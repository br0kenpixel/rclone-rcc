use crate::commands::cp;
use std::path::PathBuf;

pub fn move_(
    dir: PathBuf,
    file: PathBuf,
    dest: PathBuf,
    password: String,
    salt: Option<String>,
    reverse: bool,
) -> i32 {
    /*
        By default this command should move a file from the encrypted directory to a location outside of it.
        If `reverse` is `true`, the file should be moved from outside the encrypted directory into it.
    */

    cp(dir, file, dest, password, salt, reverse, true)
}
