use jpostcode_rs::lookup_address;

fn main() {
    match lookup_address("1000001") {
        Ok(addresses) => {
            for addr in addresses {
                println!("基本フォーマット: {}", addr.formatted());
                println!("\nカナ付きフォーマット:\n{}", addr.formatted_with_kana());
            }
        }
        Err(e) => eprintln!("Error: {}", e),
    }
}
