use std::fs;
use std::fmt::Write;


fn main() {
    let path = "/home/Dspivey/Programming/rust_projects/c_compiler/return_2.c";
    let infile = fs::read_to_string(path).expect("Unable to read file");

    generate_assembly(parse(lex(&infile)));
}

//Lexing enums
#[derive(Debug, PartialEq)]
enum Token {
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
    Invalid(String),
    Empty,
}

#[derive(Debug, PartialEq)]
enum KeywordType {
    Return,
    Int,
}

//Parsing enums
#[derive(Debug)]
enum Program {
    FunctionDeclaration(String),
    Statement(StatementType),
    Expression(ExpressionType),
}

#[derive(Debug)]
enum StatementType {
    Return,
}

#[derive(Debug)]
enum ExpressionType {
    Constant(u64),
}

impl Program {
    //This may not be the best way to do this
    fn parse_function_declaration(mut tokens: Vec<Token>, mut ast: Vec<Program>) -> (Vec<Token>, Vec<Program>) {
        let mut function_identifier = String::new();
        if let Token::Identifier(identifier) = &tokens[1] {
            function_identifier = identifier.to_string();
        }
        //let function_identifier_copy = function_identifier.clone();

        let left: Vec<Token> = vec![
            Token::Keyword(KeywordType::Int),
            Token::Identifier(function_identifier.to_owned()),
            Token::OpenParenthesis,
            Token::CloseParenthesis,
            Token::OpenBrace,
        ];
        let right = Token::CloseBrace;

        for (i, left_token) in left.iter().enumerate() {
            let token = &tokens[i];
            if left_token != token {
                panic!("Type a proper function declaration.  you moron");
            }
        }

        if right != tokens[tokens.len() - 1] {
            panic!("yo mama's a hoe");
        }

        for _ in left.iter().enumerate() {
            tokens.remove(0);
        }
        tokens.remove(tokens.len() - 1);

        println!("\n<tokens after function parse>");
        for token in tokens.iter() {
            println!("{:?}", token);
        }

        ast.push(Program::FunctionDeclaration(function_identifier));
        (tokens, ast)
    }

    fn parse_statement(tokens: Vec<Token>, mut ast: Vec<Program>) -> Vec<Program> {
        let mut last_token = &Token::Empty;
        for token in tokens.iter() {
            match token {
                Token::Keyword(KeywordType::Return) => {
                    match last_token {
                        Token::Empty => last_token = token,
                        _ => panic!("Return must come first in a statement"),
                    }
                }
                Token::IntegerLiteral(_) => {
                    match last_token {
                        Token::Keyword(KeywordType::Return) |
                        Token::Negation |
                        Token::BitwiseComplement |
                        Token::LogicalNegation
                        => last_token = token,
                        _ => panic!("Improper order of integer literal"),
                    }
                }
                Token::Semicolon => {
                    match last_token {
                        Token::IntegerLiteral(_) => (),
                        _ => panic!("Semicolon must come last in a statement"),
                    }
                }
                Token::Negation |
                Token::BitwiseComplement |
                Token::LogicalNegation
                => {
                    match last_token {
                        //FIXME: could pose a problem if return keyword is entered between operators
                        Token::Keyword(KeywordType::Return) |
                        Token::Negation |
                        Token::BitwiseComplement |
                        Token::LogicalNegation
                        => last_token = token,
                        _ => panic!("Improper order of operator"),
                    }
                }
                _ => panic!("Invalid token"),
            }
        }

        if tokens[tokens.len() - 1] != Token::Semicolon {
            panic!("you forgot a semicolon LOLLLLLLLLLLL");
        }

        //FIXME: Not expandable.  Need conditions to determine what is pushed to the AST
        //FIXME: yep yep you gotta support the uh the uh new stuff ohmygod please for the love of God
        //FIXME: GO TO FREAKING SLEEP
        if let Token::IntegerLiteral(integer_literal) = tokens[1] {
            ast.push(Program::Statement(StatementType::Return));
            ast.push(Program::Expression(ExpressionType::Constant(integer_literal)));
        }

        ast
    }
}

fn lex(infile: &String) -> Vec<Token> {
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
                //FIXME: change these to or (|) conditions so as not to repeat the same code each time.
                '(' => {
                    letters.push(letter.to_owned());
                    letters.push(char.to_string());
                    letter = String::new();
                    continue;
                }
                ')' => {
                    letters.push(letter.to_owned());
                    letters.push(char.to_string());
                    letter = String::new();
                    continue;
                }
                '{' => {
                    letters.push(letter.to_owned());
                    letters.push(char.to_string());
                    letter = String::new();
                    continue;
                }
                '}' => {
                    letters.push(letter.to_owned());
                    letters.push(char.to_string());
                    letter = String::new();
                    continue;
                }
                ';' => {
                    letters.push(letter.to_owned());
                    letters.push(char.to_string());
                    letter = String::new();
                    continue;
                }
                '-' => {
                    letters.push(letter.to_owned());
                    letters.push(char.to_string());
                    letter = String::new();
                    continue;
                }
                '~' => {
                    letters.push(letter.to_owned());
                    letters.push(char.to_string());
                    letter = String::new();
                    continue;
                }
                '!' => {
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

fn parse(lexer: Vec<Token>) -> Vec<Program> {
    let ast: Vec<Program> = Vec::new();

    let fun_decl = Program::parse_function_declaration(lexer, ast);
    let tokens = fun_decl.0;
    let ast = fun_decl.1;

    let statement = Program::parse_statement(tokens, ast);
    let ast = statement;

    println!("\n<ast>");
    for node in &ast {
        println!("{:?}", node);
    }

    ast
}

//assembly for return_2.c
/*
    .globl	main
main:
    movl	$2, %eax
    ret
*/

fn generate_assembly(ast: Vec<Program>) {
    let mut file_string = String::new();

    for (i, node) in ast.iter().enumerate() {
        match node {
            Program::FunctionDeclaration(_) => {
                file_string.push_str("
                    .globl main
                main:
                ")
            }
            Program::Statement(StatementType::Return) => {
                match ast[i + 1] {
                    Program::Expression(ExpressionType::Constant(num)) => {
                        write!(file_string, "
                    movl    ${}, %eax
                    ret
                    ", num).unwrap();
                    }
                    _ => (),
                }
            }
            _ => (),
        }
    }
    println!("{}", file_string);
    fs::write("/home/Dspivey/Programming/rust_projects/c_compiler/asm.s", file_string).expect("could not generate assembly file!!");
}