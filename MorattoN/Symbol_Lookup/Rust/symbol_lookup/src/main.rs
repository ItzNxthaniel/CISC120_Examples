use crate::symbols::{SymbolData, to_titlecase};
use std::io::Write;
use std::process;

mod symbols;

fn find_symbol(symbols: &[SymbolData], lookup_symbol: char) -> Option<SymbolData> {
    let found_symbol = symbols
        .iter()
        .find(|symbol_data| symbol_data.symbol == lookup_symbol);

    found_symbol.cloned()
}

fn find_symbol_by_name(symbols: &[SymbolData], lookup_name: &str) -> Option<SymbolData> {
    symbols.iter().find_map(|symbol_data| {
        if symbol_data.given_name == lookup_name || symbol_data.aliases.contains(&lookup_name) {
            Some(symbol_data.clone())
        } else {
            None
        }
    })
}

fn main() {
    let mut new_run: bool = true;

    let symbols: [SymbolData; 32] = symbols::get_symbols();
    loop {
        let mut user_input = String::new();

        while user_input.is_empty() {
            if new_run {
                print!("Enter a symbol to look up (exit to quit / all to list all symbols): ");
            } else {
                print!("Invalid input, please try again: ")
            }

            new_run = false;
            std::io::stdout().flush().expect("Failed to flush stdout");
            std::io::stdin()
                .read_line(&mut user_input)
                .expect("Failed to read line");

            user_input = user_input.trim().to_lowercase();
        }

        match user_input.len() {
            0 => panic!("How'd you get here?"),
            1 => {
                let user_char = user_input
                    .chars()
                    .next()
                    .expect("Failed to get first character");

                let symbol = find_symbol(&symbols, user_char);
                if let Some(symbol_data) = symbol {
                    println!("{}", symbol_data)
                } else {
                    println!("Sorry, '{}' is an invalid symbol.", user_char);
                }

                println!();
                new_run = true;
            }
            _ => {
                match user_input.as_str() {
                    "exit" => {
                        println!("Goodbye!");
                        process::exit(0)
                    }
                    "all" => {
                        println!("All symbols:");
                        for symbol in &symbols {
                            println!("  {} - {}", symbol.symbol, to_titlecase(symbol.given_name));
                        }
                    }
                    _ => {
                        let symbol = find_symbol_by_name(&symbols, user_input.as_str());

                        if let Some(symbol_data) = symbol {
                            println!("\nSymbol found:");
                            println!("{}", symbol_data);
                        } else {
                            println!("Sorry, '{}', is an invalid symbol.", user_input);
                        }
                    }
                }

                println!();
                new_run = true;
            }
        }
    }
}
