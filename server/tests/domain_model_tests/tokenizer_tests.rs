// pub fn add(a: i32, b: i32) -> i32 {
//     a + b
// }

// This is a really bad adding function, its purpose is to fail in this
// example.
// #[allow(dead_code)]
// fn bad_add(a: i32, b: i32) -> i32 {
//     a - b
// }

#[cfg(test)]
mod tests {
    use server::domain_model::tokenizer::{Token, tokenize_line};
    // Note this useful idiom: importing names from outer (for mod tests) scope.

    #[test]
    fn test_tokenizer_one_dot() {
        let input = ".";
        let mut output: Vec<Token> = Vec::new();
        output.push(Token::Dot);

        assert_eq!(tokenize_line(input.to_string()), Some(output));
    }

    #[test]
    fn test_tokenizer_one_dash() {
        let input = "-";
        let mut output: Vec<Token> = Vec::new();
        output.push(Token::Dash);

        assert_eq!(tokenize_line(input.to_string()), Some(output));
    }

    #[test]
    fn test_tokenizer_one_left_arrow() {
        let input = "<";
        let mut output: Vec<Token> = Vec::new();
        output.push(Token::LeftArrow);

        assert_eq!(tokenize_line(input.to_string()), Some(output));
    }

    #[test]
    fn test_tokenizer_one_right_arrow() {
        let input = ">";
        let mut output: Vec<Token> = Vec::new();
        output.push(Token::RightArrow);

        assert_eq!(tokenize_line(input.to_string()), Some(output));
    }

    #[test]
    fn test_tokenizer_one_identifier_char() {
        let input_1 = "w";
        let input_2 = "word";
        let input_3 = "word with spaces";

        let mut output_1: Vec<Token> = Vec::new();
        output_1.push(Token::Identifier("w".to_string()));
        let mut output_2: Vec<Token> = Vec::new();
        output_2.push(Token::Identifier("word".to_string()));
        let mut output_3: Vec<Token> = Vec::new();
        output_3.push(Token::Identifier("word with spaces".to_string()));

        assert_eq!(tokenize_line(input_1.to_string()), Some(output_1));
        assert_eq!(tokenize_line(input_2.to_string()), Some(output_2));
        assert_eq!(tokenize_line(input_3.to_string()), Some(output_3));
    }

    #[test]
    fn test_tokenizer_one_identifier_word() {
        let input_2 = "word";

        let mut output_2: Vec<Token> = Vec::new();
        output_2.push(Token::Identifier("word".to_string()));
        

        assert_eq!(tokenize_line(input_2.to_string()), Some(output_2));
    }


    #[test]
    fn test_tokenizer_one_identifier_word_with_spaces() {
        let input_3 = "word with spaces";

        let mut output_3: Vec<Token> = Vec::new();
        output_3.push(Token::Identifier("word with spaces".to_string()));

        assert_eq!(tokenize_line(input_3.to_string()), Some(output_3));
    }
    // ADD NUMBER TESTS

    // ADD MULTIPLE TOKEN TESTS

    // ADD ESCAPE CHARACTER

}