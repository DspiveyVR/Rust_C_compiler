pub mod parser {
    pub use crate::lexer::lexer::{Token, KeywordType};

    #[derive(Debug, PartialEq)]
    pub enum Program {
        FunctionDeclaration(String),
        Statement(StatementType),
        Expression(ExpressionType),
        //FIXME: maybe stupid
        //FIXME: later i might need to replace the Empty variants (Token also has one) with optionals
        Empty,
    }

    #[derive(Debug, PartialEq)]
    pub enum StatementType {
        Return,
    }

    #[derive(Debug, PartialEq)]
    pub enum ExpressionType {
        Constant(u64),
        Negation(Box<ExpNegation>),
        //Todo
        BitwiseComplement,
        //Todo
        LogicalNegation,
        Addition(Box<ExpAdd>),
        //Todo
        Multiplication,
        //Todo
        Division,
        //FIXME: Semicolon variant is nonsense.  NOT an expression.  Remove any implementation of this
        Semicolon,
    }

    #[derive(Debug, PartialEq)]
    pub struct ExpNegation {
        complement: Option<Program>,
    }

    #[derive(Debug, PartialEq)]
    pub struct ExpAdd {
        op1: Program,
        op2: Program,
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
                            | Token::Addition
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
                let mut token_index: usize;
                match token {
                    Token::Keyword(KeywordType::Return) => {
                        let mut statement: Vec<&Token> = Vec::new();

                        token_index = i + 1;
                        while tokens[token_index] != Token::Semicolon {
                            statement.push(&tokens[token_index]);
                            token_index += 1;
                        }

                        println!("\n<statement tokens>");
                        for token in statement.iter() {
                            println!("{:?}", token);
                        }

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

        fn parse_expression(mut tokens: Vec<&Token>) -> Vec<Program> {
            let mut ast_exp: Vec<Program> = Vec::new();

            let token_index = 0;
            let called_recursively = false;
            ast_exp.push(Self::parse_expression_recursive(&mut tokens, token_index, called_recursively));

            ast_exp
        }

        fn parse_expression_recursive(mut tokens: &mut Vec<&Token>, token_index: usize, mut called_recursively: bool) -> Program {
            let mut term = Program::Empty;

            //Terms that come from recursive calls should return early.
            //Otherwise they should cascade into the next match statement.
            match tokens.get(token_index) {
                Some(&Token::IntegerLiteral(num)) => {
                    tokens.remove(token_index);
                    term = Program::Expression(ExpressionType::Constant(*num));
                    if called_recursively {
                        return term;
                    }
                }
                Some(&Token::Negation) => {
                    tokens.remove(token_index);
                    let last_called_recursively = called_recursively;
                    called_recursively = true;
                    let exp = Self::parse_expression_recursive(tokens, token_index, called_recursively);
                    term = Program::Expression(ExpressionType::Negation(Box::new(ExpNegation {
                        complement: Some(exp)
                    })));
                    if last_called_recursively {
                        return term;
                    }
                }
                _ => {
                    Self::parse_expression_recursive(tokens, token_index + 1, called_recursively);
                }
            }


            match tokens.get(token_index) {
                Some(&Token::Addition) => {
                    tokens.remove(token_index);
                    called_recursively = false;
                    let exp = Self::parse_expression_recursive(tokens, token_index, called_recursively);
                    term = Program::Expression(ExpressionType::Addition(Box::new(ExpAdd {
                        op1: term,
                        op2: exp,
                    })))
                }
                _ => (),
            }

            term
        }
    }
}