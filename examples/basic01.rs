use jpostcode_rs::{lookup_address, lookup_addresses, search_by_address, ToJson};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Lookup addresses for a postal code 🏠
    let addresses = lookup_address("0280052")?;
    for addr in addresses {
        println!("{}", addr.formatted()); // "〒0280052 岩手県久慈市本町"
        println!("{}", addr.formatted_with_kana()); // With kana reading
    }

    // Prefix-based lookup (returns all matching addresses) 🏘️
    let prefix_matches = lookup_addresses("028")?;
    for addr in prefix_matches {
        println!("{}", addr.formatted());
    }

    // Search by address text (supports both Japanese and kana) 🔍
    let results = search_by_address("札幌");
    for addr in results {
        println!("Found: {}", addr.full_address());
    }

    // JSON serialization 📄
    let addresses = lookup_address("0280052")?;
    let json = addresses.to_json()?;
    println!("{}", json);

    // Available address fields
    if let Some(addr) = addresses.first() {
        println!("Postcode: {}", addr.postcode);
        println!("Prefecture: {} ({})", addr.prefecture, addr.prefecture_kana);
        println!("City: {} ({})", addr.city, addr.city_kana);
        println!("Town: {} ({})", addr.town, addr.town_kana);
    }

    Ok(())
}
