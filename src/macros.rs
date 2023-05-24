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

pub(crate) use create_cipher;
