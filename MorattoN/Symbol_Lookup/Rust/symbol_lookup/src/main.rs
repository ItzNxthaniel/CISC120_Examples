use std::io::Write;
use crate::symbols::SymbolData;

mod symbols;

fn find_symbol(symbols: &[SymbolData], lookup_symbol: char) -> Option<SymbolData> {
    let found_symbol = symbols.iter()
        .find(|symbol_data| symbol_data.symbol == lookup_symbol);

    found_symbol.cloned()
}

fn main() {
    let mut first_run: bool = true;

    let symbols: Vec<SymbolData> = symbols::get_symbols();
    let mut user_input = String::new();

    while user_input.len() <= 0 {
        if first_run {
            print!("Enter a symbol to look up (exit to quit / all to list all symbols): ");
        } else {
            print!("Invalid input, please try again: ")
        }
        first_run = false;
        std::io::stdout().flush().expect("Failed to flush stdout");
        std::io::stdin().read_line(&mut user_input).expect("Failed to read line");

        user_input = user_input.trim().to_string();
    }

    match user_input.len() {
        0 => panic!("How'd you get here?"),
        1 => {
            let user_char = user_input.chars().next().expect("Failed to get first character");

            let symbol = find_symbol(&symbols, user_char);
            if let Some(symbol_data) = symbol {
                println!("{}", symbol_data)
            }
        },
        _ => {
            unimplemented!();
        }
    }
}
