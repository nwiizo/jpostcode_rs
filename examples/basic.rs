use jpostcode_rs::lookup_address;

fn main() {
    match lookup_address("1000001") {
        Ok(addresses) => {
            for addr in addresses {
                println!(
                    "〒{} {} {} {} ({} {} {})",
                    addr.postcode,
                    addr.prefecture,
                    addr.city,
                    addr.town,
                    addr.prefecture_kana,
                    addr.city_kana,
                    addr.town_kana
                );
            }
        }
        Err(e) => eprintln!("Error: {}", e),
    }
}
