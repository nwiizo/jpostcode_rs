use once_cell::sync::Lazy;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;

mod error;
pub use error::JPostError;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Address {
    #[serde(rename = "postcode")]
    pub postcode: String,
    #[serde(rename = "prefecture")]
    pub prefecture: String,
    #[serde(rename = "prefecture_kana")]
    pub prefecture_kana: String,
    #[serde(rename = "prefecture_code")]
    pub prefecture_code: i32,
    #[serde(rename = "city")]
    pub city: String,
    #[serde(rename = "city_kana")]
    pub city_kana: String,
    #[serde(rename = "town")]
    pub town: String,
    #[serde(rename = "town_kana")]
    pub town_kana: String,
    #[serde(default, rename = "street")]
    pub street: Option<String>,
    #[serde(default, rename = "office_name")]
    pub office_name: Option<String>,
    #[serde(default, rename = "office_name_kana")]
    pub office_name_kana: Option<String>,
}

pub trait ToJson {
    fn to_json(&self) -> Result<String, serde_json::Error>;
}

impl ToJson for Address {
    fn to_json(&self) -> Result<String, serde_json::Error> {
        serde_json::to_string(self)
    }
}

impl ToJson for Vec<Address> {
    fn to_json(&self) -> Result<String, serde_json::Error> {
        serde_json::to_string(self)
    }
}

impl Address {
    pub fn full_address(&self) -> String {
        format!("{}{}{}", self.prefecture, self.city, self.town)
    }

    pub fn full_address_kana(&self) -> String {
        format!(
            "{}{}{}",
            self.prefecture_kana, self.city_kana, self.town_kana
        )
    }

    pub fn formatted(&self) -> String {
        format!("〒{} {}", self.postcode, self.full_address())
    }

    pub fn formatted_with_kana(&self) -> String {
        format!(
            "〒{}\n{}\n{}",
            self.postcode,
            self.full_address(),
            self.full_address_kana()
        )
    }
}

static ADDRESS_MAP: Lazy<HashMap<String, Vec<Address>>> = Lazy::new(|| {
    let data = include_str!(concat!(env!("OUT_DIR"), "/address_data.json"));
    let raw_map: HashMap<String, Value> =
        serde_json::from_str(data).expect("Failed to parse raw data");

    let mut result: HashMap<String, Vec<Address>> = HashMap::new();

    for (k, v) in raw_map {
        match v {
            Value::Array(arr) => {
                let addresses: Vec<Address> = arr
                    .into_iter()
                    .filter_map(|addr_value| serde_json::from_value(addr_value).ok())
                    .collect();
                if !addresses.is_empty() {
                    result.insert(k, addresses);
                }
            }
            Value::Object(_) => {
                if let Ok(addr) = serde_json::from_value(v) {
                    result.insert(k, vec![addr]);
                }
            }
            _ => continue,
        }
    }
    result
});

pub fn lookup_address(postal_code: &str) -> Result<Vec<Address>, JPostError> {
    if !is_valid_postal_code(postal_code) {
        return Err(JPostError::InvalidFormat);
    }
    ADDRESS_MAP
        .get(postal_code)
        .cloned()
        .ok_or(JPostError::NotFound)
}

pub fn lookup_addresses(postal_code: &str) -> Result<Vec<Address>, JPostError> {
    if !is_valid_postal_code_prefix(postal_code) {
        return Err(JPostError::InvalidFormat);
    }

    let matches: Vec<Address> = ADDRESS_MAP
        .iter()
        .filter(|(k, _)| k.starts_with(postal_code))
        .flat_map(|(_, addresses)| addresses.clone())
        .collect();

    if matches.is_empty() {
        Err(JPostError::NotFound)
    } else {
        Ok(matches)
    }
}

pub fn search_by_address(query: &str) -> Vec<Address> {
    ADDRESS_MAP
        .values()
        .flat_map(|addresses| {
            addresses
                .iter()
                .filter(|addr| {
                    addr.full_address().contains(query) || addr.full_address_kana().contains(query)
                })
                .cloned()
        })
        .collect()
}

pub fn valid_postal_code(code: &str) -> bool {
    is_valid_postal_code(code)
}

fn is_valid_postal_code(code: &str) -> bool {
    code.len() == 7 && code.chars().all(|c| c.is_ascii_digit())
}

fn is_valid_postal_code_prefix(code: &str) -> bool {
    code.len() >= 3 && code.len() <= 7 && code.chars().all(|c| c.is_ascii_digit())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_lookup_address_multiple() {
        let results = lookup_address("0280052").unwrap();
        assert!(!results.is_empty());
        assert!(results.iter().all(|addr| addr.postcode == "0280052"));
    }

    #[test]
    fn test_lookup_addresses() {
        let results = lookup_addresses("028").unwrap();
        assert!(!results.is_empty());
        assert!(results.iter().all(|addr| addr.postcode.starts_with("028")));
    }

    #[test]
    fn test_invalid_format() {
        assert!(matches!(
            lookup_address("123"),
            Err(JPostError::InvalidFormat)
        ));
    }

    #[test]
    fn test_search_by_address() {
        let results = search_by_address("岩手県");
        assert!(!results.is_empty());
        assert!(results.iter().all(|addr| addr.prefecture == "岩手県"));
    }

    #[test]
    fn test_address_formatting() {
        let addr = Address {
            postcode: "0280052".to_string(),
            prefecture: "岩手県".to_string(),
            prefecture_kana: "イワテケン".to_string(),
            prefecture_code: 3,
            city: "久慈市".to_string(),
            city_kana: "クジシ".to_string(),
            town: "本町".to_string(),
            town_kana: "ホンチョウ".to_string(),
            street: None,
            office_name: None,
            office_name_kana: None,
        };

        assert_eq!(addr.full_address(), "岩手県久慈市本町");
        assert_eq!(addr.full_address_kana(), "イワテケンクジシホンチョウ");
        assert_eq!(addr.formatted(), "〒0280052 岩手県久慈市本町");
    }
}
