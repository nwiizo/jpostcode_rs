# jpostcode_rs 📮

Japanese postal code lookup library in Rust 🗾

## Features 🚀
- Fast postal code lookup with pre-compiled address data ⚡
- Single and multiple address lookups 🔍
- Address search (Japanese and kana) 🔤
- JSON serialization support 📋
- Multiple address format styles 📝
- No runtime data loading required 📦

## Usage 💻

```rust
use jpostcode_rs::{lookup_address, lookup_addresses, search_by_address, ToJson};

// Single address lookup 🏠
let address = lookup_address("0280052")?;
println!("{}", address.formatted());  // "〒0280052 岩手県久慈市本町"

// Multiple address lookup 🏘️
let addresses = lookup_addresses("028")?;

// Search by address text 🔍
let results = search_by_address("札幌");

// JSON output 📄
let json = address.to_json()?;
```

## Data Source 📚
Data from [jpostcode-data](https://github.com/kufu/jpostcode-data) 🗃️

## License 📜
MIT License
