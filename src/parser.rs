pub mod parser {
    pub use crate::lexer::lexer::{Token, KeywordType};

    #[derive(Debug, PartialEq)]
    pub enum Program {
        FunctionDeclaration(String),
        Statement(StatementType),
        Expression(ExpressionType),
        Empty,
    }

    #[derive(Debug, PartialEq)]
    pub enum StatementType {
        Return,
    }

    #[derive(Debug, PartialEq)]
    //TODO: Perhaps add separate enums to create a distinction between "prime" expressions and ones with "parameters"
    pub enum ExpressionType {
        //FIXME: Oh dooky cheeks they all gotta be recursive
        Constant(u64, Box<Option<Program>>),
        Negation,
        BitwiseComplement,
        LogicalNegation,
        //Todo
        Addition(Box<Program>, Box<Program>),
        //Todo
        Multiplication,
        //Todo
        Division,
        //FIXME: Semicolon variant is nonsense.  NOT an expression.  Remove any implementation of this
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
                            | Token::Addition
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
                    Token::Addition => {
                        match last_token {
                            Token::IntegerLiteral(_) => last_token = token,
                            _ => panic!("you can't add that stoopit")
                        }
                    }
                    _ => panic!("Invalid token"),
                }
            }

            if tokens[tokens.len() - 1] != Token::Semicolon {
                panic!("you forgot a semicolon LOLLLLLLLLLLL");
            }



            for (i, token) in tokens.iter().enumerate() {
                let mut token_index:usize;
                match token {
                    Token::Keyword(KeywordType::Return) => {
                        let mut statement: Vec<&Token> = Vec::new();
                        //let mut ast_borrow: Vec<&Program> = Vec::new();
                        token_index = i + 1;
                        while tokens[token_index] != Token::Semicolon {
                            statement.push(&tokens[token_index]);
                            token_index += 1;
                        }
                        // for node in ast.iter() {
                        //     ast_borrow.push(node);
                        // }
                        let ast_exp = Self::parse_expression(statement);
                        for node in ast_exp {
                            ast.push(node);
                        }
                        ast.push(Program::Statement(StatementType::Return));
                    }
                    _ => (),
                }
            }

            ast
        }

        fn parse_expression(tokens: Vec<&Token>) -> Vec<Program> {
            let mut ast_exp: Vec<Program> = Vec::new();
            //Add expressions to list by order of operations, run through lists linearly to determine parameters to expressions
            let mut primary_expressions: Vec<&Token> = Vec::new();
            let mut secondary_expressions: Vec<&Token> = Vec::new();

            for token in tokens {
                match token {
                    //Secondary expressions
                    Token::Addition => secondary_expressions.push(token),
                    //Primary expressions
                    Token::IntegerLiteral(_) => primary_expressions.push(token),
                    _ => (),
                }
            }

            println!("\n<primary expressions>");
            for expression in primary_expressions.iter() {
                println!("{:?}", expression);
            }

            println!("\n<secondary expressions>");
            for expression in secondary_expressions.iter() {
                println!("{:?}", expression);
            }

            for expression in secondary_expressions {
                match expression {
                    Token::Addition => {
                        let op1;
                        let op2;
                        let mut expressions = primary_expressions.iter();
                        match expressions.next() {
                            Some(Token::IntegerLiteral(num)) => {
                                op1 = Program::Expression(ExpressionType::Constant(*num, Box::new(None)));
                            }
                            _ => op1 = Program::Empty,
                        }
                        match expressions.next() {
                            Some(Token::IntegerLiteral(num)) => {
                                op2 = Program::Expression(ExpressionType::Constant(*num, Box::new(None)));
                            }
                            _ => op2 = Program::Empty,
                        }
                        primary_expressions.remove(0);
                        primary_expressions.remove(0);
                        ast_exp.push(Program::Expression(ExpressionType::Addition(Box::new(op1), Box::new(op2))));
                    }
                    _ => (),
                }
            }

            for expression in primary_expressions {
                match expression {
                    Token::IntegerLiteral(num) => ast_exp.push(Program::Expression(ExpressionType::Constant(*num, Box::new(None)))),
                    _ => (),
                }
            }

            ast_exp
        }
    }
}