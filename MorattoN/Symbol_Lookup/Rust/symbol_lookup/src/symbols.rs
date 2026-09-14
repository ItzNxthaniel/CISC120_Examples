use std::fmt;
use std::fmt::Formatter;

#[derive(Clone)]
pub struct SymbolData {
    pub symbol: char,
    pub given_name: &'static str,
    pub aliases: [&'static str; 4],
    pub alias_count: i8,
}

impl SymbolData {
    pub fn new(
        symbol: char,
        given_name: &'static str,
        aliases: [&'static str; 4],
        alias_count: i8,
    ) -> Self {
        Self {
            symbol,
            given_name,
            aliases,
            alias_count,
        }
    }
}

impl fmt::Display for SymbolData {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self.alias_count {
            0 => write!(
                f,
                "  Symbol: {}\n  Given Name: {}",
                self.symbol,
                to_titlecase(self.given_name)
            ),
            _ => write!(
                f,
                "  Symbol: {}\n  Given Name: {}\n  Aliases: {}",
                self.symbol,
                to_titlecase(self.given_name),
                self.aliases
                    .iter()
                    .map(|alias| to_titlecase(alias))
                    .filter(|alias| !alias.is_empty())
                    .collect::<Vec<String>>()
                    .join(", ")
            ),
        }
    }
}

pub fn to_titlecase(string: &str) -> String {
    let words = string.split_whitespace();
    let mut final_string = Vec::new();
    for word in words {
        let final_word = match word.get(0..1) {
            Some(first_char) => first_char.to_uppercase().to_string() + &word[1..],
            None => String::from(word),
        };

        final_string.push(final_word);
    }

    final_string.join(" ")
}

pub fn get_symbols() -> [SymbolData; 32] {
    [
        SymbolData::new('`', "backtick", ["backquote", "grave accent", "", ""], 2),
        SymbolData::new('~', "tilde", ["", "", "", ""], 0),
        SymbolData::new( '!', "exclamation mark", ["exclamation point", "bang", "", ""], 2,),
        SymbolData::new('@', "at symbol", ["at sign", "at", "", ""], 2),
        SymbolData::new('#', "pound", ["number", "hashtag", "sh", ""], 3),
        SymbolData::new('$', "dollar", ["", "", "", ""], 0),
        SymbolData::new('%', "percent", ["mod", "modulo", "", ""], 2),
        SymbolData::new('^', "caret", ["hat", "exponent", "", ""], 2),
        SymbolData::new('&', "ampersand", ["and", "", "", ""], 1),
        SymbolData::new('*', "asterisk", ["star", "", "", ""], 1),
        SymbolData::new('(', "open parenthesis", ["left parenthesis", "", "", ""], 1),
        SymbolData::new( ')', "close parenthesis", ["right parenthesis", "", "", ""], 1,),
        SymbolData::new('-', "hyphen", ["dash", "minus", "subtract", ""], 3),
        SymbolData::new('_', "underscore", ["understrike", "", "", ""], 1),
        SymbolData::new('=', "equals", ["", "", "", ""], 0),
        SymbolData::new('+', "plus", ["addition", "", "", ""], 1),
        SymbolData::new('[', "open bracket", ["left bracket", "", "", ""], 1),
        SymbolData::new(']', "close bracket", ["right bracket", "", "", ""], 1),
        SymbolData::new( '{', "open curly bracket", ["left curly bracket", "", "", ""], 1,),
        SymbolData::new( '}', "close curly bracket", ["right curly bracket", "", "", ""], 1,),
        SymbolData::new( '|', "pipe", ["vertical bar", "vertical line", "broken bar", ""], 3,),
        SymbolData::new('\\', "backslash", ["reverse slash", "", "", ""], 1),
        SymbolData::new(';', "semicolon", ["", "", "", ""], 0),
        SymbolData::new(':', "colon", ["", "", "", ""], 0),
        SymbolData::new('\'', "apostrophe", ["single quote", "", "", ""], 1),
        SymbolData::new('"', "double quote", ["", "", "", ""], 0),
        SymbolData::new(',', "comma", ["", "", "", ""], 0),
        SymbolData::new('.', "period", ["dot", "", "", ""], 1),
        SymbolData::new( '<', "open angle bracket", ["less than", "left angle bracket", "", ""], 2,),
        SymbolData::new( '>', "close angel bracket", ["greater than", "right angle bracket", "", ""], 2,),
        SymbolData::new('/', "forward slash", ["", "", "", ""], 0),
        SymbolData::new('?', "question mark", ["", "", "", ""], 0),
    ]
}
