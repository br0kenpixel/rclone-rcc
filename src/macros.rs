macro_rules! create_cipher {
    ($cipher_var: ident, $password: expr, $salt: expr) => {
        let spinner = Spinner::new(spinners::Dots, "Creating cipher...", Color::White);
        let $cipher_var = match Cipher::new($password, $salt) {
            Ok(c) => c,
            Err(e) => {
                spinner.fail(&format!("Failed to create cipher: {e}"));
                return 1;
            }
        };
        spinner.success("Created cipher");
    };
}

#[cfg(feature = "mount")]
macro_rules! into_fuse_err {
    ($e: expr, $error: expr) => {
        $e.ok_or($error)?
    };
}

#[cfg(feature = "mount")]
macro_rules! into_fuse_result {
    ($e: expr) => {
        $e.map_err(|err| Errno::from_i32(err.raw_os_error().unwrap_or(0)))
    };
}

pub(crate) use create_cipher;
#[cfg(feature = "mount")]
pub(crate) use into_fuse_err;
#[cfg(feature = "mount")]
pub(crate) use into_fuse_result;
