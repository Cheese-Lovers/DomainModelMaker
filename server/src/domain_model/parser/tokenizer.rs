use std::{fmt, str::Chars};

use serde::{Deserialize, Serialize}; 

#[derive(Clone, PartialEq, Debug, Serialize, Deserialize)]
pub enum Token {
    Keyword(Keyword),
    Identifier(String),
    LeftArrow,
    RightArrow,
    Dash,
    NaturalNumber(usize),
    Float(f32),
    Range,
    Colon,
    EndStatement
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum Keyword {
    Pin
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Token::Identifier(string) => write!(f, "{}", string),
            Token::Keyword(keyword) => write!(f, "{}", keyword),
            Token::LeftArrow => write!(f, "<"),
            Token::RightArrow => write!(f, ">"),
            Token::Dash => write!(f, "-"),
            Token::NaturalNumber(n) => write!(f, "{}", n),
            Token::Float(x) => write!(f, "{}", x),
            Token::Range => write!(f, ".."),
            Token::Colon => write!(f, ":"),
            Token::EndStatement => writeln!(f),
        }
    }
}

impl fmt::Display for Keyword {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Keyword::Pin => write!(f, "pin")
        }
    }
}

pub struct TokenParsingIterator<'a> {
    chars: itertools::structs::PeekNth<Chars<'a>>,
}

impl TokenParsingIterator<'_> {
    pub fn new(input: &str) -> TokenParsingIterator<'_> {
        TokenParsingIterator { chars: itertools::peek_nth(input.chars()) }
    }
}

#[derive(PartialEq, Eq, Debug, Serialize, Deserialize)]
pub enum ParseTokenError {
    /// The user meant to write a range ("..") but only wrote a single dot
    SawSingleDot,
    /// The user included a dot in a number, but then included another dot after that (e.g. "1.2.3")
    TwoDotsInNumber,
    /// The user forgot to close an identifier
    UnterminatedIdentifier,
    /// The user used a tilde to indicate a negative number, but did not include a number after the tilde
    NoNumberAfterTilde,
    /// The user used a backslash to escape a character, but did not include a character after the backslash
    NoCharacterAfterEscape
}

impl std::fmt::Display for ParseTokenError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ParseTokenError::SawSingleDot => write!(f, "Saw single dot. Did you mean to write a range (\"..\")?"),
            ParseTokenError::TwoDotsInNumber => write!(f, "Two dots in number. Did you mean to write a range (\"..\")?"),
            ParseTokenError::UnterminatedIdentifier => write!(f, "Unterminated identifier. Did you forget to close it?"),
            ParseTokenError::NoNumberAfterTilde => write!(f, "No number after tilde. Did you forget to include a number after the tilde?"),
            ParseTokenError::NoCharacterAfterEscape => write!(f, "No character after escape. Did you forget to include a character after the backslash?"),
        }
    }
}

impl Iterator for TokenParsingIterator<'_> {
    type Item = Result<Token, ParseTokenError>;

    fn next(&mut self) -> Option<Self::Item> {
        while self.chars.peek().is_some_and(|c| *c == ' ') {
            self.chars.next(); // Skip leading whitespace
        }

        match self.chars.next()? {
            // Dots and Ranges
            '.' => {
                let Some('.') = self.chars.peek() else {
                    return Some(Err(ParseTokenError::SawSingleDot));
                };
                self.chars.next(); // Consume the second dot
                Some(Ok(Token::Range))
            }
            // Symbols
            '-' => Some(Ok(Token::Dash)),
            '<' => Some(Ok(Token::LeftArrow)),
            '>' => Some(Ok(Token::RightArrow)),
            ':' => Some(Ok(Token::Colon)),
            '\n' => Some(Ok(Token::EndStatement)),
            // Numbers
            symbol @ ('~' | '0'..='9') => parse_number_token(symbol, &mut self.chars),
            // Identifiers
            symbol => parse_identifier_token(symbol, &mut self.chars)
        }
    }
}

fn parse_identifier_token(first_char: char, chars: &mut itertools::structs::PeekNth<Chars>) -> Option<Result<Token, ParseTokenError>> {
    #[derive(Clone, Copy)]
    enum IdentifierKind {
        DoubleQuote,
        Apostrophe,
        Tick,
        Bare
    }

    use IdentifierKind::*;

    let mut buffer = String::new();

    let start = match first_char {
        '\\' => {
            let Some(next_char) = chars.next() else {
                return Some(Err(ParseTokenError::NoCharacterAfterEscape));
            };
            buffer.push(next_char); // Take the next character as a literal
            Bare
        },
        '"' => DoubleQuote,
        '\'' => Apostrophe,
        '`' => Tick,
        _ => {
            buffer.push(first_char);
            Bare
        }
    };

    loop {
        if 
            let Bare = start &&
            let None | Some(' ' | '-' | '.' | '<' | '>' | ':' | '\n' | '0'..='9') = chars.peek() &&
            let Some(keyword) = try_parse_keyword(&buffer) 
        {
            return Some(Ok(Token::Keyword(keyword)));
        }

        match (start, chars.peek()) {
            (DoubleQuote, Some('\"')) | (Apostrophe, Some('\'')) | (Tick, Some('`')) => {
                chars.next(); // Consume the closing quote
                return Some(Ok(Token::Identifier(buffer.trim().to_string())));
            },
            (DoubleQuote | Apostrophe | Tick, None) => {
                return Some(Err(ParseTokenError::UnterminatedIdentifier));
            }
            (Bare, None | Some('-' | '.' | '<' | '>' | ':' | '\n' | '0'..='9')) => {
                return Some(Ok(Token::Identifier(buffer.trim().to_string())));
            },
            (_, Some('\\')) => {
                chars.next(); // Consume the backslash
            },
            _ => {} // Keep accumulating characters
        }

        buffer.push(chars.next()?);
    }
}

fn parse_number_token(first_char: char, chars: &mut itertools::structs::PeekNth<Chars>) -> Option<Result<Token, ParseTokenError>> {
    let mut buffer = String::new();
    buffer.push(first_char);

    loop {
        match chars.peek() {
            Some('.') => {
                if let Some('.') = chars.peek_nth(1) {
                    // This is the start of a range, not a decimal point.
                    return Some(parse_number_from_buffer(&buffer))
                }
                if contains_decimal_point(&buffer) {
                    return Some(Err(ParseTokenError::TwoDotsInNumber))
                }
                buffer.push(chars.next()?);
            },
            Some('0'..='9') => {
                buffer.push(chars.next()?);
            },
            _ => return Some(parse_number_from_buffer(&buffer))
        }
    }
}

fn parse_number_from_buffer(buffer: &str) -> Result<Token, ParseTokenError> {
    let Some(number) = buffer.strip_prefix('~') else {
        if contains_decimal_point(buffer) {
            return Ok(Token::Float(buffer.parse().unwrap()))
        } else {
            return Ok(Token::NaturalNumber(buffer.parse().unwrap()))
        }
    };
    // Since the number can be negative, it must be a float
    let Some(number) = number.parse::<f32>().ok() else {
        return Err(ParseTokenError::NoNumberAfterTilde)
    };
    Ok(Token::Float(-number))
}

fn contains_decimal_point(buffer: &str) -> bool {
    buffer.contains('.')
}

fn try_parse_keyword(buffer: &str) -> Option<Keyword> {
    match buffer.trim() {
        "pin" => Some(Keyword::Pin),
        _ => None
    }
}

#[cfg(test)]
mod tests {
    use crate::domain_model::parser::tokenizer::ParseTokenError;
    use super::{Token, TokenParsingIterator};

    fn tokenize(input: &str) -> Result<Vec<Token>, ParseTokenError> {
        TokenParsingIterator::new(input).collect::<Result<Vec<Token>, _>>()
    }

    #[test]
    fn test_tokenizer_not_one_dot() {
        let input = ".";
        let output: Vec<Token> = vec![Token::Range];

        assert_ne!(tokenize(input), Ok(output));
    }

    #[test]
    fn test_tokenizer_one_dash() {
        let input = "-";
        let output: Vec<Token> = vec![Token::Dash];

        assert_eq!(tokenize(input), Ok(output));
    }

    #[test]
    fn test_tokenizer_one_left_arrow() {
        let input = "<";
        let output: Vec<Token> = vec![Token::LeftArrow];

        assert_eq!(tokenize(input), Ok(output));
    }

    #[test]
    fn test_tokenizer_one_right_arrow() {
        let input = ">";
        let output: Vec<Token> = vec![Token::RightArrow];

        assert_eq!(tokenize(input), Ok(output));
    }

    #[test]
    fn test_tokenizer_one_identifier_char() {
        let input_1 = "w";
        let input_2 = "word";
        let input_3 = "word with spaces";

        let output_1: Vec<Token> = vec![Token::Identifier("w".to_string())];
        let output_2: Vec<Token> = vec![Token::Identifier("word".to_string())];
        let output_3: Vec<Token> = vec![Token::Identifier("word with spaces".to_string())];

        assert_eq!(tokenize(input_1), Ok(output_1));
        assert_eq!(tokenize(input_2), Ok(output_2));
        assert_eq!(tokenize(input_3), Ok(output_3));
    }

    #[test]
    fn test_tokenizer_one_identifier_word() {
        let input_2 = "word";

        let output_2: Vec<Token> = vec![Token::Identifier("word".to_string())];

        assert_eq!(tokenize(input_2), Ok(output_2));
    }


    #[test]
    fn test_tokenizer_one_identifier_word_with_spaces() {
        let input_3 = "word with spaces";

        let output_3: Vec<Token> = vec![Token::Identifier("word with spaces".to_string())];

        assert_eq!(tokenize(input_3), Ok(output_3));
    }

    #[test]
    fn test_tokenizer_multiline() {
        let input_3 = "peter-knows-wendy\nhook-knows-peter";

        let output_3: Vec<Token> = vec![
            Token::Identifier("peter".to_string()),
            Token::Dash,
            Token::Identifier("knows".to_string()),
            Token::Dash,
            Token::Identifier("wendy".to_string()),
            Token::EndStatement,
            Token::Identifier("hook".to_string()),
            Token::Dash,
            Token::Identifier("knows".to_string()),
            Token::Dash,
            Token::Identifier("peter".to_string())
        ];

        assert_eq!(tokenize(input_3), Ok(output_3));
    }
    // ADD NUMBER TESTS

    // ADD MULTIPLE TOKEN TESTS

    // ADD ESCAPE CHARACTER

}