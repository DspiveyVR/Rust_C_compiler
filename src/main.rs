use std::fs;
use std::fs::File;
use std::fmt::Write;

//FIXME: install tabnine in idea
fn main() {
    let path = "/home/Dspivey/Programming/rust_projects/c_compiler/return_2.c";
    let infile = fs::read_to_string(path).expect("Unable to read file");

    generate_assembly(parse(&infile));
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
    Invalid(String),
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
    fn parse_function_declaration(mut tokens: Vec<Token>, mut ast: Vec<Program>) -> (Vec<Token>, Vec<Program>) {
        let mut function_identifier = String::new();
        if let Token::Identifier(identifier) = &tokens[1] {
            function_identifier = identifier.to_string();
        }
        let function_identifier_copy = function_identifier.clone();

        let left: Vec<Token> = vec![
            Token::Keyword(KeywordType::Int),
            Token::Identifier(function_identifier),
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

        for token in tokens.iter() {
            println!("{:?}", token);
        }

        ast.push(Program::FunctionDeclaration(function_identifier_copy));
        (tokens, ast)
    }

    fn parse_statement(mut tokens: Vec<Token>, mut ast: Vec<Program>) -> Vec<Program> {
        let statement: Vec<Token> = vec![
            Token::Keyword(KeywordType::Return),
            //1 is arbitrary.  data is not compared
            Token::IntegerLiteral(1),
            Token::Semicolon,
        ];
        for (i, token) in tokens.iter().enumerate() {
            assert_eq!(std::mem::discriminant(&statement[i]), std::mem::discriminant(token), "oh cwap");
        }

        if let Token::IntegerLiteral(integer_literal) = tokens[1] {
            ast.push(Program::Statement(StatementType::Return));
            ast.push(Program::Expression(ExpressionType::Constant(integer_literal)));
        }

        ast
    }
}

fn lex(infile: &String) -> Vec<Token> {
    let mut tokens: Vec<Token> = Vec::new();
    let mut infile_tmp = String::from(infile);

    //Parses the string into individual tokens, assigning each a Token data type
    while infile_tmp.len() > 0 {
        let mut severed = 0;
        let mut word: String = String::new();
        for char in infile_tmp.chars() {
            severed += 1;
            if (char == ' ') || char.is_control() {
                break;
            }
            word.push(char);
        }
        while severed > 0 {
            infile_tmp.remove(0);
            severed -= 1;
        }
        if &word != &String::from("") {
            match word.as_str() {
                "int" => tokens.push(Token::Keyword(KeywordType::Int)),
                "main()" =>
                    {
                        //FIXME: parentheses crap.  doodoo solution works now but will not be viable later
                        let mut open_parenthesis = false;
                        let mut close_parenthesis = false;
                        let mut counter = 0;
                        for char in word.chars() {
                            if char == '(' {
                                word.remove(counter);
                                open_parenthesis = true;
                                break;
                            } else {
                                counter += 1;
                            }
                        }

                        let mut counter_1 = 0;
                        for char in word.chars() {
                            if char == ')' {
                                word.remove(counter_1);
                                close_parenthesis = true;
                                break;
                            } else {
                                counter_1 += 1;
                            }
                        }

                        tokens.push(Token::Identifier(word));
                        //Determines the order of the parentheses
                        //FIXME: parentheses screw all up when out of order or if there is whitespace between main and parentheses
                        if counter <= counter_1 {
                            tokens.push(Token::OpenParenthesis);
                            tokens.push(Token::CloseParenthesis);
                        } else {
                            tokens.push(Token::CloseParenthesis);
                            tokens.push(Token::OpenParenthesis);
                        }
                    }
                "{" => tokens.push(Token::OpenBrace),
                "}" => tokens.push(Token::CloseBrace),
                "return" => tokens.push(Token::Keyword(KeywordType::Return)),
                word if word.as_bytes()[0].is_ascii_digit() => {
                    //FIXME: only works for single-digit integers (could use a for-if instead)
                    tokens.push(Token::IntegerLiteral(word.as_bytes()[0] as u64 - '0' as u64));
                    tokens.push(Token::Semicolon);
                }
                _ => tokens.push(Token::Invalid(word)),
            }
        }
    }
    tokens
}

fn parse(infile: &String) -> Vec<Program> {
    let mut ast: Vec<Program> = Vec::new();

    for token in lex(&infile) {
        println!("{:?}", token);
    }
    println!("\n");

    let fun_decl = Program::parse_function_declaration(lex(&infile), ast);
    let tokens = fun_decl.0;
    let ast = fun_decl.1;

    let statement = Program::parse_statement(tokens, ast);
    let ast = statement;

    println!("\n");
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
    let mut value: u64 = 0;

    if let Program::Expression(ExpressionType::Constant(num)) = &ast[2] {
        value = *num;
    };
    for node in &ast {
        match (node) {
            Program::FunctionDeclaration(_) => {
                file_string.push_str("
                    .globl main
                main:
                ")
            }
            Program::Statement(StatementType::Return) => {
                write!(file_string, "
                    movl    ${}, %eax
                    ret
                ", value).unwrap();
            }
            _ => (),
        }
    }
    println!("{}", file_string);
    fs::write("/home/Dspivey/Programming/rust_projects/c_compiler/asm.s", file_string).expect("could not generate assembly file!!");
}