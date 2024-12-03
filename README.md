# jpostcode_rs 📮

Japanese postal code lookup library in Rust 🗾

## Features 🚀
- Fast postal code lookup with pre-compiled address data ⚡
- Multiple address support for single postal code 🏘️
- Prefix-based address lookup 🔍
- Address search (Japanese and kana) 🔤
- JSON serialization support 📋
- Multiple address format styles 📝
- No runtime data loading required 📦

## Usage 💻

```rust
use jpostcode_rs::{lookup_address, lookup_addresses, search_by_address, ToJson};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Lookup addresses for a postal code 🏠
    let addresses = lookup_address("0280052")?;
    for addr in addresses {
        println!("{}", addr.formatted());  // "〒0280052 岩手県久慈市本町"
        println!("{}", addr.formatted_with_kana());  // With kana reading
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
```

## Address Formats 📝

```rust
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addresses = lookup_address("0280052")?;
    if let Some(addr) = addresses.first() {
        // Basic format
        println!("{}", addr.formatted());
        // Output: 〒0280052 岩手県久慈市町

        // Format with kana
        println!("{}", addr.formatted_with_kana());
        // Output: 
        // 〒0280052
        // 岩手県久慈市本町
        // イワテケンクジシホンチョウ

        // Individual components
        println!("{}", addr.full_address());        // 岩手県久慈市本町
        println!("{}", addr.full_address_kana());   // イワテケンクジシホンチョウ
    }
    Ok(())
}
```

## Error Handling 🚨

The library provides two types of errors:
- `JPostError::InvalidFormat` - When postal code format is invalid
- `JPostError::NotFound` - When no addresses are found for the given postal code

```rust
fn main() {
    match lookup_address("invalid") {
        Ok(addresses) => {
            for addr in addresses {
                println!("{}", addr.formatted());
            }
        }
        Err(JPostError::InvalidFormat) => {
            eprintln!("Invalid postal code format");
        }
        Err(JPostError::NotFound) => {
            eprintln!("No addresses found");
        }
    }
}
```

## Data Source 📚
Data from [jpostcode-data](https://github.com/kufu/jpostcode-data) 🗃️

## License 📜
MIT License
