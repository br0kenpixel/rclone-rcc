use rclone_crypt::obscure::obscure as obscure_text;

pub fn obscure(value: String) -> i32 {
    match obscure_text(&value) {
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
