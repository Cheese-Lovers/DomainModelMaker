use std::{iter::Peekable, num::NonZeroUsize};

use serde::{Deserialize, Serialize};

use crate::domain_model::{graph::{Arrow, Multiplicity}, parser::tokenizer::{Keyword, Token}};


pub enum Statement {
    NewRelation {
        text: Option<String>,
        weight: NonZeroUsize,
        entity_1: String,
        entity_2: String,
        arrow_1: Arrow,
        arrow_2: Arrow,
        mult_1: Multiplicity,
        mult_2: Multiplicity
    },
    Pin {
        entity: String,
        x: f32,
        y: f32
    }
}

impl Statement {
    pub fn try_from_tokens(tokens: &[Token]) -> Result<Statement, ParseStatementError> {
        // Assume everything is a new relation for now
        let mut tokens = tokens.iter().peekable();

        match tokens.peek() {
            Some(Token::Keyword(Keyword::Pin)) => {
                tokens.next(); // Consume the "pin" keyword
                Self::try_from_pin_tokens(&mut tokens)
            },
            Some(Token::Identifier(entity_1)) => {
                tokens.next(); // Consume the first identifier
                Self::try_from_relation_tokens(entity_1, &mut tokens)
            },
            token => Err(ParseStatementError::ExpectedIdentifier(token.cloned().cloned()))
        }
    }

    pub fn try_from_pin_tokens<'a>(tokens: &mut Peekable<impl Iterator<Item=&'a Token>>) -> Result<Statement, ParseStatementError> {
        let entity = match tokens.next() {
            Some(Token::Identifier(entity)) => entity,
            token => return Err(ParseStatementError::ExpectedIdentifier(token.cloned()))
        };

        match tokens.peek() {
            Some(Token::Colon) => {
                tokens.next(); // Consume the colon
            },
            token => return Err(ParseStatementError::ExpectedColon(token.cloned().cloned()))
        }

        let x = match tokens.next() {
            Some(Token::NaturalNumber(x)) => *x as f32,
            Some(Token::Float(x)) => *x,
            token => return Err(ParseStatementError::ExpectedCoordinate(token.cloned()))
        };

        let y = match tokens.next() {
            Some(Token::NaturalNumber(y)) => *y as f32,
            Some(Token::Float(y)) => *y,
            token => return Err(ParseStatementError::ExpectedCoordinate(token.cloned()))
        };

        match tokens.next() {
            None | Some(Token::EndStatement) => {},
            Some(token) => return Err(ParseStatementError::ExpectedEndOfStatement(token.clone()))
        }

        Ok(Statement::Pin { entity: entity.clone(), x, y })
    }

    pub fn try_from_relation_tokens<'a>(entity_1: &'a str, tokens: &mut Peekable<impl Iterator<Item=&'a Token>>) -> Result<Statement, ParseStatementError> {
        let entity_1 = entity_1.to_string();

        let arrow_1 = {
            if let Some(Token::LeftArrow) = tokens.peek() {
                tokens.next(); // Consume the left arrow
                Arrow::Arrow
            } else if let Some(Token::RightArrow) = tokens.peek() {
                return Err(ParseStatementError::ArrowInWrongDirection)
            } else {
                Arrow::None
            }
        };

        let mult_1 = parse_multiplicity(tokens)?;

        let weight_1 = count_dashes(tokens);

        let text = match tokens.peek() {
            Some(Token::Identifier(text)) => {
                tokens.next(); // Consume the identifier
                Ok(text)
            },
            token => Err(ParseStatementError::ExpectedIdentifier(token.cloned().cloned())),
        };

        if tokens.peek().is_none() {
            // The ident we just read wasn't text---it was actually the second entity.

            let text = text?;

            let Some(weight) = NonZeroUsize::new(weight_1) else {
                return Err(ParseStatementError::NoWeightSpecified);
            };

            return Ok(
                Statement::NewRelation {
                    text: None,
                    weight,
                    entity_1: entity_1.clone(),
                    entity_2: text.clone(),
                    arrow_1,
                    arrow_2: Arrow::None,
                    mult_1,
                    mult_2: Multiplicity::None
                }
            )
        }

        let weight_2 = count_dashes(tokens);

        let mult_2 = parse_multiplicity(tokens)?;

        let arrow_2 = {
            if let Some(Token::RightArrow) = tokens.peek() {
                tokens.next(); // Consume the right arrow
                Arrow::Arrow
            } else if let Some(Token::LeftArrow) = tokens.peek() {
                return Err(ParseStatementError::ArrowInWrongDirection)
            } else {
                Arrow::None
            }
        };

        let entity_2 = match tokens.next() {
            Some(Token::Identifier(entity_2)) => entity_2,
            token => {
                return Err(ParseStatementError::ExpectedIdentifier(token.cloned()))
            }
        };

        match tokens.next() {
            None | Some(Token::EndStatement) => {},
            Some(token) => {
                return Err(ParseStatementError::ExpectedEndOfStatement(token.clone()))
            }
        }

        let weight = usize::max(weight_1, weight_2);

        let Some(weight) = NonZeroUsize::new(weight) else {
            return Err(ParseStatementError::NoWeightSpecified);
        };

        Ok(
            Statement::NewRelation {
                text: text.ok().cloned(),
                weight,
                entity_1: entity_1.clone(),
                entity_2: entity_2.clone(),
                arrow_1,
                arrow_2,
                mult_1,
                mult_2
            }
        )
    }
}

fn count_dashes<'a>(tokens: &mut Peekable<impl Iterator<Item=&'a Token>>) -> usize {
    let mut weight = 0;
    while let Some(Token::Dash) = tokens.peek() {
        tokens.next(); // Consume the dash
        weight += 1;
    }
    weight
}

fn parse_multiplicity<'a>(tokens: &mut Peekable<impl Iterator<Item=&'a Token>>) -> Result<Multiplicity, ParseStatementError> {
    let num = match tokens.peek() {
        Some(Token::NaturalNumber(num)) => num,
        Some(token @ Token::Float(..)) => {
            return Err(ParseStatementError::ExpectedNaturalNumber((*token).clone()));
        }
        _ => return Ok(Multiplicity::None)
    };
    tokens.next(); // Consume the number

    let Some(Token::Range) = tokens.peek() else {
        return Ok(Multiplicity::Number(*num));
    };
    tokens.next(); // Consume the dot

    let num_2 = match tokens.peek() {
        Some(Token::NaturalNumber(num_2)) => num_2,
        Some(token @ Token::Float(..)) => {
            return Err(ParseStatementError::ExpectedNaturalNumber((*token).clone()));
        }
        _ => return Ok(Multiplicity::RangeFrom(*num..))
    };
    tokens.next(); // Consume the second number

    Ok(Multiplicity::Range(*num..*num_2))
}

#[derive(PartialEq, Debug, Serialize, Deserialize)]
pub enum ParseStatementError {
    /// The parser expected an identifier but got something else (or nothing at all)
    ExpectedIdentifier(Option<Token>),
    /// The parser expected a colon but got something else (or nothing at all)
    ExpectedColon(Option<Token>),
    /// The parser expected a coordinate but got something else (or nothing at all)
    ExpectedCoordinate(Option<Token>),
    /// The parser encountered an arrow in the wrong direction
    ArrowInWrongDirection,
    /// Finished parsing before the end of the statement was reached
    /// Perhaps there's an extraneous identifier?
    ExpectedEndOfStatement(Token),
    /// No dashes were supplied
    NoWeightSpecified,
    /// The parser expected a natural number but got a float instead
    ExpectedNaturalNumber(Token)
}

impl std::fmt::Display for ParseStatementError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ParseStatementError::ExpectedIdentifier(token) => write!(f, "Expected identifier, got {:?}", token),
            ParseStatementError::ExpectedColon(token) => write!(f, "Expected colon, got {:?}", token),
            ParseStatementError::ExpectedCoordinate(token) => write!(f, "Expected coordinate, got {:?}", token),
            ParseStatementError::ArrowInWrongDirection => write!(f, "Arrow in wrong direction"),
            ParseStatementError::ExpectedEndOfStatement(token) => write!(f, "Expected end of statement, got {:?}", token),
            ParseStatementError::NoWeightSpecified => write!(f, "No weight specified. Please include at least one dash ('-') to indicate the weight of the relation."),
            ParseStatementError::ExpectedNaturalNumber(token) => write!(f, "Expected natural number, got {:?}", token)
        }
    }
}

pub struct StatementParsingIterator<'a, I: Iterator<Item=&'a Token>> {
    tokens: Peekable<I>,
}

impl<'a, I: Iterator<Item=&'a Token>> StatementParsingIterator<'a, I> {
    pub fn new(tokens: I) -> StatementParsingIterator<'a, I> {
        StatementParsingIterator { tokens: tokens.peekable() }
    }
}

impl<'a, I: Iterator<Item=&'a Token>> Iterator for StatementParsingIterator<'a, I> {
    type Item = Result<Statement, ParseStatementError>;

    fn next(&mut self) -> Option<Self::Item> {
        let mut statement_tokens = Vec::<Token>::new();

        while matches!(self.tokens.peek(), Some(&Token::EndStatement)) {
            self.tokens.next();
        }

        for token in self.tokens.by_ref() {
            if let Token::EndStatement = token {
                break;
            }
            statement_tokens.push(token.clone());
        }

        if statement_tokens.is_empty() {
            None
        } else {
            Some(Statement::try_from_tokens(&statement_tokens))
        }
    }
}

#[cfg(test)]
pub mod test {
    use crate::domain_model::parser::tokenizer::{ParseTokenError, TokenParsingIterator};

    use super::*;

    fn parse_statement(input: &str) -> Result<Statement, ParseStatementError> {
        let tokens = {
            TokenParsingIterator::new(input)
                .collect::<Result<Vec<Token>, ParseTokenError>>()
                .expect("Tokenization failed in test")
        };

        Statement::try_from_tokens(&tokens)
    }

    macro_rules! assert_statement_parsing {
        ($input:expr) => {
            let result = parse_statement($input);
            assert!(result.is_ok(), "{}", result.err().unwrap());
        };
    }

    macro_rules! assert_parsing_failed {
        ($input:expr) => {
            let result = parse_statement($input);
            assert!(result.is_err());
        };
    }

    #[test]
    fn unnamed_relation() {
        assert_statement_parsing!("a-b");
    }

    #[test]
    fn named_relation() {
        assert_statement_parsing!("a-loves-b");
    }

    #[test]
    fn no_weight_relation() {
        assert_parsing_failed!("a1loves1b");
    }

    #[test]
    fn directed_relation() {
        assert_statement_parsing!("a->b");
    }

    #[test]
    fn reverse_directed_relation() {
        assert_statement_parsing!("a<-b");
    }

    #[test]
    fn wrong_way_directed_relation() {
        assert_parsing_failed!("a-<b");
    }

    #[test]
    fn wrong_way_reverse_directed_relation() {
        assert_parsing_failed!("a>-b");
    }
}