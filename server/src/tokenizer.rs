use std::fmt; 

pub enum Token {
    Identifier(String),
    LeftArrow,
    RightArrow,
    Dash,
    Number(usize),
    Dot,
    Escape,
    Error(String) // for testing purposes
}

impl fmt::Debug for Token {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Token::Identifier(string) => write!(f, "ID \"{}\"", string),
            Token::LeftArrow => write!(f, "L_Arrow"),
            Token::RightArrow => write!(f, "R_Arrow"),
            Token::Dash => write!(f, "Dash"),
            Token::Number(n) => write!(f, "Num {}", n),
            Token::Dot => write!(f, "Dot"),
            Token::Escape => write!(f, "Escape"),
            Token::Error(err) => write!(f, "{}", err),
        }
    }
}

pub enum State {
    Identifier(String),
    Else(char),
    Number(String)
}


// Smelliest Code in the Universe
// NEEDS FIXING: THE LAST TOKEN IS NOT ACCOUNTED FOR
pub fn tokenize_line(input: String) -> Option<Vec<Token>> {
    let mut output = Vec::new();
    let mut state: State;
    
    let c = input.chars().next();
    
    state = match c {
        None => return None,
        Some('-') | Some('.') | Some('<') | Some('>') | Some('\\') => State::Else('\n'), // newline char will not be added to the list of tokens
        Some(x) if x.is_numeric() => State::Number(String::new()),
        _ => State::Identifier(String::new())
    };  

    for c in input.chars() {

        match state {
            State::Identifier(ref s) => {
                state = match c {
                    '-' | '.' | '<' | '>' | '\\' | '\n' => {
                        output.push(Token::Identifier(s.clone()));
                        State::Else(c)
                    },
                    x if x.is_numeric() => {
                        output.push(Token::Identifier(s.clone()));
                        State::Number(c.to_string())
                    },
                    _ => State::Identifier(s.clone() + &c.to_string())
                }
            },
            State::Number(ref n) => {
                state = match c {
                    '-' | '.' | '<' | '>' | '\\' | '\n' => {
                        output.push(Token::Number(n.parse().unwrap()));
                        State::Else(c)
                    },
                    x if x.is_numeric() => {
                        State::Number(n.clone() + &c.to_string())
                    },
                    _ => {
                        output.push(Token::Number(n.parse().unwrap()));
                        State::Identifier(c.to_string())
                    }
                }
            }
            State::Else(ch) => {
                output.push(generate_else_token(ch));
                
                state = match c {
                    '-'| '.' | '<' | '>' | '\\' | '\n' => {
                        State::Else(c)
                    },
                    x if x.is_numeric() => {
                        State::Number(c.to_string())
                    },
                    _ => {
                        State::Identifier(c.to_string())
                    }
                }
            }
        }
    }

    output.push(
        match state {
            State::Else(ch) => generate_else_token(ch),
            State::Number(n) => Token::Number(n.parse().unwrap()),
            State::Identifier(s) => Token::Identifier(s.clone()),
        }
    );

    Some(output)
}

fn generate_else_token(ch: char) -> Token {
    match ch {
        '-' => Token::Dash,
        '.' => Token::Dot,
        '<' => Token::LeftArrow,
        '>' => Token::RightArrow,
        '\\' => Token::Escape,
        '\n' => Token::Identifier(String::new()),
        _ => Token::Error("Error, unrecognized symbol ".to_string() + &ch.to_string())
    }
}