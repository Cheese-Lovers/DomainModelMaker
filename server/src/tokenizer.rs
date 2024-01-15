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

pub enum State {
    Identifier(String),
    Else(char),
    Number(String)
}


// Smelliest Code in the Universe
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
                    '-' | '.' | '<' | '>' | '\\' => {
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
                    '-' | '.' | '<' | '>' | '\\' => {
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
                output.push(
                    match ch {
                        '-' => Token::Dash,
                        '.' => Token::Dot,
                        '<' => Token::LeftArrow,
                        '>' => Token::RightArrow,
                        '\\' => Token::Escape,
                        '\n' => Token::Identifier(String::new()),
                        _ => Token::Error("Error, unrecognized symbol ".to_string() + &ch.to_string())
                    }
                );
                    
                state = match c {
                    '-'| '.' | '<' | '>' | '\\' => {
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

    Some(output)
}