use rclone_crypt::obscure::reveal as reveal_text;

pub fn reveal(value: String) -> i32 {
    match reveal_text(&value) {
        Ok(result) => {
            println!("{result}");
            0
        }
        Err(e) => {
            eprintln!("Error: {e}");
            1
        }
    }
}
