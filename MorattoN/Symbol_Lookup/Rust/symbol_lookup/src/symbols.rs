use std::fmt;
use std::fmt::Formatter;

#[derive(Clone)]
pub struct SymbolData {
    pub symbol: char,
    pub given_name: &'static str,
    pub aliases: Vec<&'static str>,
}

impl SymbolData {
    pub fn new(symbol: char, given_name: &'static str, aliases: Vec<&'static str>) -> Self {
        Self { symbol, given_name, aliases }
    }
}

impl fmt::Display for SymbolData {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self.aliases.len() {
            0 => write!(f, "  Symbol: {}\n  Given Name: {}", self.symbol, self.given_name),
            _ => write!(f, "  Symbol: {}\n  Given Name: {}\n  Aliases: {}",
                        self.symbol,
                        to_uppercase(self.given_name),
                        self.aliases
                            .iter()
                            .map(|alias| to_uppercase(alias))
                            .collect::<Vec<String>>()
                            .join(", "))
        }
    }
}

fn to_uppercase(string: &str) -> String {
    let words = string.split_whitespace();
    let mut final_string = Vec::new();
    for word in words {
        let final_word = match word.get(0..1) {
            Some(first_char) => first_char.to_uppercase().to_string() + &word[1..],
            None => String::from(word)
        };

        final_string.push(final_word);
    }

    final_string.join(" ")
}

pub fn get_symbols() -> Vec<SymbolData> {
    Vec::from([
        SymbolData::new('`',  "tilde",               Vec::new()),
        SymbolData::new('~',  "backtick",            Vec::from(["backquote", "grave accent"])),
        SymbolData::new('!',  "exclamation mark",    Vec::from(["exclamation point"])),
        SymbolData::new('@',  "at symbol",           Vec::from(["at sign", "at"])),
        SymbolData::new('#',  "pound",               Vec::from(["number", "hashtag"])),
        SymbolData::new('$',  "dollar",              Vec::new()),
        SymbolData::new('%',  "percent",             Vec::from(["mod\\modulo"])),
        SymbolData::new('^',  "caret",               Vec::from(["hat", "exponent"])),
        SymbolData::new('&',  "ampersand",           Vec::from(["and"])),
        SymbolData::new('*',  "asterisk",            Vec::from(["star"])),
        SymbolData::new('(',  "open parenthesis",    Vec::from(["left parenthesis"])),
        SymbolData::new(')',  "close parenthesis",   Vec::from(["right parenthesis"])),
        SymbolData::new('-',  "hyphen",              Vec::from(["dash", "minus/subtract"])),
        SymbolData::new('_',  "underscore",          Vec::from(["understrike"])),
        SymbolData::new('=',  "equals",              Vec::new()),
        SymbolData::new('+',  "plus",                Vec::from(["addition"])),
        SymbolData::new('[',  "open bracket",        Vec::from(["left bracket"])),
        SymbolData::new(']',  "close bracket",       Vec::from(["right bracket"])),
        SymbolData::new('{',  "open curly bracket",  Vec::from(["left curly bracket"])),
        SymbolData::new('}',  "close curly bracket", Vec::from(["right curly bracket"])),
        SymbolData::new('|',  "pipe",                Vec::from(["vertical bar", "vertical line", "broken bar"])),
        SymbolData::new('\\', "backslash",           Vec::from(["reverse slash"])),
        SymbolData::new(';',  "semicolon",           Vec::new()),
        SymbolData::new(':',  "colon",               Vec::new()),
        SymbolData::new('\'', "apostrophe",          Vec::from(["single quote"])),
        SymbolData::new('"',  "double quote",        Vec::new()),
        SymbolData::new(',',  "comma",               Vec::new()),
        SymbolData::new('.',  "period",              Vec::from(["dot"])),
        SymbolData::new('<',  "open angle bracket",  Vec::from(["less than", "left angle bracket"])),
        SymbolData::new('>',  "close angel bracket", Vec::from(["greater than", "right angle bracket"])),
        SymbolData::new('/',  "forward slash",       Vec::new()),
        SymbolData::new('?',  "question mark",       Vec::new()),
    ])
}