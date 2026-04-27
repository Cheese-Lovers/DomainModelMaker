use std::{iter::Peekable, num::NonZeroUsize};

use serde::{Deserialize, Serialize};

use crate::domain_model::{graph::{Arrow, Multiplicity}, parser::tokenizer::Token};


pub enum Statement {
    NewRelation {
        text: String,
        weight: NonZeroUsize,
        entity_1: String,
        entity_2: String,
        arrow_1: Arrow,
        arrow_2: Arrow,
        mult_1: Multiplicity,
        mult_2: Multiplicity
    }
}

impl Statement {
    pub fn try_from_tokens(tokens: &[Token]) -> Result<Statement, ParseStatementError> {
        // Assume everything is a new relation for now
        let mut tokens = tokens.iter().peekable();

        let entity_1 = match tokens.next() {
            Some(Token::Identifier(entity_1)) => entity_1,
            token => {
                return Err(ParseStatementError::ExpectedIdentifier(token.cloned()))
            }
        };

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

        let mult_1 = parse_multiplicity(&mut tokens);

        let weight_1 = count_dashes(&mut tokens);

        let text = {
            if let Some(Token::Identifier(text)) = tokens.peek() {
                tokens.next(); // Consume the text token
                text.clone()
            } else {
                String::new()
            }
        };

        if tokens.peek().is_none_or(|tok| matches!(tok, Token::EndStatement)) {
            // The ident we just read wasn't text---it was actually the second entity.

            let Some(weight) = NonZeroUsize::new(weight_1) else {
                return Err(ParseStatementError::NoWeightSpecified);
            };

            return Ok(
                Statement::NewRelation {
                    text: String::new(),
                    weight,
                    entity_1: entity_1.clone(),
                    entity_2: text.clone(),
                    arrow_1,
                    arrow_2: Arrow::None,
                    mult_1,
                    mult_2: Multiplicity::Number(1)
                }
            )
        }

        let weight_2 = count_dashes(&mut tokens);

        let mult_2 = parse_multiplicity(&mut tokens);

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
                text,
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

fn parse_multiplicity<'a>(tokens: &mut Peekable<impl Iterator<Item=&'a Token>>) -> Multiplicity {
    let Some(Token::Number(num)) = tokens.peek() else {
        return Multiplicity::Number(1);
    };
    tokens.next(); // Consume the number

    let Some(Token::Range) = tokens.peek() else {
        return Multiplicity::Number(*num);
    };
    tokens.next(); // Consume the dot

    let Some(Token::Number(num_2)) = tokens.peek() else {
        return Multiplicity::RangeFrom(*num..);
    };
    tokens.next(); // Consume the second number

    Multiplicity::Range(*num..*num_2)
}

#[derive(PartialEq, Eq, Debug, Serialize, Deserialize)]
pub enum ParseStatementError {
    /// The parser expected an identifier but got something else (or nothing at all)
    ExpectedIdentifier(Option<Token>),
    /// The parser encountered an arrow in the wrong direction
    ArrowInWrongDirection,
    /// Finished parsing before the end of the statement was reached
    /// Perhaps there's an extraneous identifier?
    ExpectedEndOfStatement(Token),
    /// No dashes were supplied
    NoWeightSpecified
}

impl std::fmt::Display for ParseStatementError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ParseStatementError::ExpectedIdentifier(token) => write!(f, "Expected identifier, got {:?}", token),
            ParseStatementError::ArrowInWrongDirection => write!(f, "Arrow in wrong direction"),
            ParseStatementError::ExpectedEndOfStatement(token) => write!(f, "Expected end of statement, got {:?}", token),
            ParseStatementError::NoWeightSpecified => write!(f, "No weight specified. Please include at least one dash ('-') to indicate the weight of the relation.")
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