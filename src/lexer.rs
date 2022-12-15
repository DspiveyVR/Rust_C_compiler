pub mod lexer {
    #[derive(Debug, PartialEq)]
    pub enum Token {
        Keyword(KeywordType),
        OpenBrace,
        CloseBrace,
        OpenParenthesis,
        CloseParenthesis,
        Semicolon,
        Identifier(String),
        IntegerLiteral(u64),
        Negation,
        BitwiseComplement,
        LogicalNegation,
        //Todo
        Addition,
        //Todo
        Multiplication,
        //Todo
        Division,
        Invalid(String),
        Empty,
    }

    #[derive(Debug, PartialEq)]
    pub enum KeywordType {
        Return,
        Int,
    }

    pub fn lex(infile: &String) -> Vec<Token> {
        let mut words: Vec<String> = Vec::new();
        let mut letters: Vec<String> = Vec::new();
        let mut tokens: Vec<Token> = Vec::new();
        let mut infile_tmp = String::from(infile);

        while infile_tmp.len() > 0 {
            //Separates file into "words" by whitespace and control chars
            let mut severed = 0;
            let mut word: String = String::new();
            for char in infile_tmp.chars() {
                severed += 1;
                if (char == ' ') || char.is_control() {
                    break;
                }
                word.push(char);
            }
            words.push(word);
            while severed > 0 {
                infile_tmp.remove(0);
                severed -= 1;
            }
        }

        //Letters are words separated into important parts, such as names and parentheses
        for word in words.iter() {
            let mut letter = String::new();
            for char in word.chars() {
                match char {
                    '('
                    | ')'
                    | '{'
                    | '}'
                    | ';'
                    | '-'
                    | '~'
                    | '!'
                    | '+'
                    => {
                        letters.push(letter.to_owned());
                        letters.push(char.to_string());
                        letter = String::new();
                        continue;
                    }
                    _ => letter.push(char),
                }
            }
            letters.push(letter);
        }

        //Parses the string into individual tokens, assigning each a Token data type
        for letter in letters.iter() {
            if letter != &String::from("") {
                match letter.as_str() {
                    "int" => tokens.push(Token::Keyword(KeywordType::Int)),
                    "main" => tokens.push(Token::Identifier(letter.to_owned())),
                    "(" => tokens.push(Token::OpenParenthesis),
                    ")" => tokens.push(Token::CloseParenthesis),
                    "{" => tokens.push(Token::OpenBrace),
                    "}" => tokens.push(Token::CloseBrace),
                    "return" => tokens.push(Token::Keyword(KeywordType::Return)),
                    //Logic to account for multiple decimal places
                    letter if letter.as_bytes()[0].is_ascii_digit() => {
                        let mut int_literal: u64 = 0;
                        let mut power_of_ten = (letter.to_string().len() - 1) as i32;
                        let mut is_integer = true;
                        for char in letter.chars() {
                            if char.is_ascii_digit() {
                                int_literal += (char as u64 - '0' as u64) * 10_u64.pow(power_of_ten as u32);
                                power_of_ten -= 1;
                            } else {
                                tokens.push(Token::Invalid("not an integer".to_string()));
                                is_integer = false;
                            }
                        }
                        if is_integer {
                            tokens.push(Token::IntegerLiteral(int_literal));
                        }
                    }
                    ";" => tokens.push(Token::Semicolon),
                    "-" => tokens.push(Token::Negation),
                    "~" => tokens.push(Token::BitwiseComplement),
                    "!" => tokens.push(Token::LogicalNegation),
                    "+" => tokens.push(Token::Addition),
                    _ => tokens.push(Token::Invalid(letter.to_owned())),
                }
            }
        }
        println!("<tokens>");
        for token in tokens.iter() {
            println!("{:?}", token);
        }

        tokens
    }
}