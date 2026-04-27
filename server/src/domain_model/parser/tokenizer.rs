use std::{fmt, iter::Peekable, str::Chars};

use serde::{Deserialize, Serialize}; 

#[derive(Clone, PartialEq, Eq, Debug, Serialize, Deserialize)]
pub enum Token {
    Identifier(String),
    LeftArrow,
    RightArrow,
    Dash,
    Number(usize),
    Range,
    Escape,
    EndStatement
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Token::Identifier(string) => write!(f, "{}", string),
            Token::LeftArrow => write!(f, "<"),
            Token::RightArrow => write!(f, ">"),
            Token::Dash => write!(f, "-"),
            Token::Number(n) => write!(f, "{}", n),
            Token::Range => write!(f, ".."),
            Token::Escape => write!(f, "\\"),
            Token::EndStatement => writeln!(f),
        }
    }
}

pub struct TokenParsingIterator<'a> {
    chars: Peekable<Chars<'a>>,
}

impl TokenParsingIterator<'_> {
    pub fn new(input: &str) -> TokenParsingIterator<'_> {
        TokenParsingIterator { chars: input.chars().peekable() }
    }
}

#[derive(PartialEq, Eq, Debug, Serialize, Deserialize)]
pub enum ParseTokenError {
    /// Suggests the user meant to write a range ("..") but only wrote a single dot
    SawSingleDot
}

impl std::fmt::Display for ParseTokenError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ParseTokenError::SawSingleDot => write!(f, "Saw single dot. Did you mean to write a range (\"..\")?")
        }
    }
}

impl Iterator for TokenParsingIterator<'_> {
    type Item = Result<Token, ParseTokenError>;

    fn next(&mut self) -> Option<Self::Item> {
        enum TokenParsingState {
            Identifier(String),
            Number(String)
        }

        let mut state = match self.chars.peek() {
            None => return None,
            Some('-' | '.' | '<' | '>' | '\\' | '\n') => {
                let symbol = self.chars.next().unwrap(); // Consume the symbol

                return match symbol {
                    '-' => Some(Ok(Token::Dash)),
                    '.' => {
                        if let Some('.') = self.chars.peek() {
                            self.chars.next(); // Consume the second dot
                            Some(Ok(Token::Range))
                        } else {
                            Some(Err(ParseTokenError::SawSingleDot))
                        }
                    },
                    '<' => Some(Ok(Token::LeftArrow)),
                    '>' => Some(Ok(Token::RightArrow)),
                    '\\' => Some(Ok(Token::Escape)),
                    '\n' => Some(Ok(Token::EndStatement)),
                    _ => unreachable!()
                }
            },
            Some(x) if x.is_ascii_digit() => TokenParsingState::Number(String::new()),
            _ => TokenParsingState::Identifier(String::new())
        };

        while let (Some(curr), next) = (self.chars.next(), self.chars.peek().cloned()) {
            let (TokenParsingState::Identifier(ref mut buffer) | TokenParsingState::Number(ref mut buffer)) = state;

            buffer.push(curr);

            match state {
                TokenParsingState::Identifier(ref buffer) => {
                    match next {
                        None | Some('-' | '.' | '<' | '>' | '\\' | '\n') => {
                            return Some(Ok(Token::Identifier(buffer.clone())));
                        },
                        Some(x) if x.is_ascii_digit() => {
                            return Some(Ok(Token::Identifier(buffer.clone())));
                        },
                        _ => {}
                    }
                },
                TokenParsingState::Number(ref buffer) => {
                    match next {
                        None | Some('-' | '.' | '<' | '>' | '\\' | '\n') => {
                            return Some(Ok(Token::Number((buffer.clone()).parse().unwrap())));
                        },
                        Some(x) if !x.is_ascii_digit() => {
                            return Some(Ok(Token::Number((buffer.clone()).parse().unwrap())));
                        },
                        _ => {
                            state = TokenParsingState::Number(buffer.clone())
                        }
                    }
                }
            }
        }

        None
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