use jpostcode_rs::search_by_address;
use std::io::{self, Write};

fn main() {
    println!("郵便番号検索UI (終了するには 'q' を入力)");

    loop {
        print!("\n住所を入力してください: ");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        let input = input.trim();

        if input == "q" {
            break;
        }

        if input.is_empty() {
            continue;
        }

        let results = search_by_address(input);

        match results.len() {
            0 => println!("検索結果なし"),
            n => {
                println!("\n{}件の結果が見つかりました:", n);
                for addr in results {
                    println!("\n{}", addr.formatted_with_kana());
                }
            }
        }
    }
}
