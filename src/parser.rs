
pub mod parser {
    pub use crate::lexer::lexer::{Token, KeywordType};

    #[derive(Debug, PartialEq)]
    pub enum Program {
        FunctionDeclaration(String),
        Statement(StatementType),
        Expression(ExpressionType),
    }

    #[derive(Debug, PartialEq)]
    pub enum StatementType {
        Return,
    }

    #[derive(Debug, PartialEq)]
    pub enum ExpressionType {
        Constant(u64),
        Negation,
        BitwiseComplement,
        LogicalNegation,
        Semicolon,
    }

    pub fn parse(lexer: Vec<Token>) -> Vec<Program> {
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
        println!("\n");

        ast
    }

    impl Program {
        //This may not be the best way to do this
        fn parse_function_declaration(mut tokens: Vec<Token>, mut ast: Vec<Program>) -> (Vec<Token>, Vec<Program>) {
            let mut function_identifier = String::new();
            if let Token::Identifier(identifier) = &tokens[1] {
                function_identifier = identifier.to_string();
            }

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
                            Token::Keyword(KeywordType::Return)
                            | Token::Negation
                            | Token::BitwiseComplement
                            | Token::LogicalNegation
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
                    Token::Negation
                    | Token::BitwiseComplement
                    | Token::LogicalNegation
                    => {
                        match last_token {
                            Token::Keyword(KeywordType::Return)
                            | Token::Negation
                            | Token::BitwiseComplement
                            | Token::LogicalNegation
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

            for token in tokens.iter() {
                match token {
                    Token::Keyword(KeywordType::Return) => ast.push(Program::Statement(StatementType::Return)),
                    Token::IntegerLiteral(integer_literal) => ast.push(Program::Expression(ExpressionType::Constant(*integer_literal))),
                    Token::Negation => ast.push(Program::Expression(ExpressionType::Negation)),
                    Token::BitwiseComplement => ast.push(Program::Expression(ExpressionType::BitwiseComplement)),
                    Token::LogicalNegation => ast.push(Program::Expression(ExpressionType::LogicalNegation)),
                    Token::Semicolon => ast.push(Program::Expression(ExpressionType::Semicolon)),
                    _ => (),
                }
            }

            ast
        }
    }
}