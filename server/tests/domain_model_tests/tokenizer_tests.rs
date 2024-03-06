#[cfg(test)]
mod tests {
    use server::domain_model::tokenizer::{Token, tokenize_line};
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

    #[test]
    fn test_tokenizer_one_number() {
        let input = "1";

        let mut output: Vec<Token> = Vec::new();
        output.push(Token::Number(1));

        assert_eq!(tokenize_line(input.to_string()), Some(output));
    }

    #[test]
    fn test_tokenizer_one_number_long() {
        let input = "11";

        let mut output: Vec<Token> = Vec::new();
        output.push(Token::Number(11));

        assert_eq!(tokenize_line(input.to_string()), Some(output));
    }

    // MULTI TOKEN TESTS

     #[test]
    fn test_tokenizer_mult_tokens_one() {
        let input = "11-test word<5";

        let mut output: Vec<Token> = Vec::new();
        output.push(Token::Number(11));
        output.push(Token::Dash);
        output.push(Token::Identifier("test word".to_string()));
        output.push(Token::LeftArrow);
        output.push(Token::Number(5));

        assert_eq!(tokenize_line(input.to_string()), Some(output));
    }

    #[test]
    fn test_tokenizer_mult_tokens_two() {
        let input = "test.-55>17jack";

        let mut output: Vec<Token> = Vec::new();
        output.push(Token::Identifier("test".to_string()));
        output.push(Token::Dot);
        output.push(Token::Dash);
        output.push(Token::Number(55));
        output.push(Token::LeftArrow);
        output.push(Token::Number(17));
        output.push(Token::Identifier("jack".to_string()));

        assert_eq!(tokenize_line(input.to_string()), Some(output));
    }

    #[test]
    fn test_dot_connection() {
        let input = "test.jack";

        let mut output: Vec<Token> = Vec::new();
        output.push(Token::Identifier("test.jack".to_string()));

        assert_eq!(tokenize_line(input.to_string()), Some(output));
    }

    #[test]
    fn test_escaping_character_identifier() {
        let input = "test1:\\test";

        let mut output: Vec<Token> = Vec::new();
        output.push(Token::Identifier("test".to_string()));
        output.push(Token::Number(1));
        output.push(Token::Identifier("\\test".to_string()));

        assert_eq!(tokenize_line(input.to_string()), Some(output));
    }
    #[test]
    fn test_escaping_character_dash() {
        let input = "test1:\\-test";

        let mut output: Vec<Token> = Vec::new();
        output.push(Token::Identifier("test".to_string()));
        output.push(Token::Number(1));
        output.push(Token::Identifier("-test".to_string()));

        assert_eq!(tokenize_line(input.to_string()), Some(output));
    }

    #[test]
    fn test_escaping_character_left_arrow() {
        let input = "test1:\\<test";

        let mut output: Vec<Token> = Vec::new();
        output.push(Token::Identifier("test".to_string()));
        output.push(Token::Number(1));
        output.push(Token::Identifier("<test".to_string()));

        assert_eq!(tokenize_line(input.to_string()), Some(output));
    }

    #[test]
    fn test_escaping_character_right_arrow() {
        let input = "test1:\\-test";

        let mut output: Vec<Token> = Vec::new();
        output.push(Token::Identifier("test".to_string()));
        output.push(Token::Number(1));
        output.push(Token::Identifier(">test".to_string()));

        assert_eq!(tokenize_line(input.to_string()), Some(output));
    }

    #[test]
    fn test_escaping_character_escape() {
        let input = "test1:\\\\test";

        let mut output: Vec<Token> = Vec::new();
        output.push(Token::Identifier("test".to_string()));
        output.push(Token::Number(1));
        output.push(Token::Identifier("\\test".to_string()));

        assert_eq!(tokenize_line(input.to_string()), Some(output));
    }

    #[test]
    fn test_escaping_character_dot() {
        let input = "test1:\\.test";

        let mut output: Vec<Token> = Vec::new();
        output.push(Token::Identifier("test".to_string()));
        output.push(Token::Number(1));
        output.push(Token::Identifier("\\.test".to_string()));

        assert_eq!(tokenize_line(input.to_string()), Some(output));
    }

    #[test]
    fn test_escaping_nothing() {
        let input = "test1:\\";

        let mut output: Vec<Token> = Vec::new();
        output.push(Token::Identifier("test".to_string()));
        output.push(Token::Number(1));
        output.push(Token::Identifier("\\".to_string()));

        assert_eq!(tokenize_line(input.to_string()), Some(output));
    }

}