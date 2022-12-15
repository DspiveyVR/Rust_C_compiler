pub mod code_generation {
    use std::fs;
    pub use crate::parser::parser::{Program, StatementType, ExpressionType};
    use std::fmt::Write;

    pub fn generate_assembly(ast: Vec<Program>) {
        let mut file_string = String::new();

        for node in ast.iter() {
            match node {
                //FIXME: This should exclusively be for the main function.  Need to specify "main" in enum
                Program::FunctionDeclaration(_) => file_string.push_str("\t.globl main \nmain:"),
                Program::Statement(StatementType::Return) => file_string.push_str("\n\tret\n"),
                Program::Expression(ExpressionType::Constant(num, _)) => write!(file_string, "\n\tmovl    ${}, %eax", num).unwrap(),
                Program::Expression(ExpressionType::Addition(op1, op2)) => {
                    let mut num1 = 0;
                    let mut num2 = 0;
                    if let Program::Expression(ExpressionType::Constant(num, _)) = **op1 {
                        num1 = num;
                    }
                    if let Program::Expression(ExpressionType::Constant(num, _)) = **op2 {
                        num2 = num;
                    }
                    write!(file_string, "\n\tmov    ${}, %rax\
                                         \n\tpush %rax", num1).unwrap();
                    write!(file_string, "\n\tmov    ${}, %rax\
                                         \n\tpop %rcx\
                                         \n\tadd %rcx, %rax", num2).unwrap();
                }
                _ => (),
            }
        }
        // //FIXME: can probably process this all linearly since the parser is different now
        // for (i, node) in ast.iter().enumerate() {
        //     match node {
        //         Program::FunctionDeclaration(_) => {
        //             file_string.push_str("\t.globl main \nmain:");
        //         }
        //         Program::Statement(_) => {
        //             let mut statement: Vec<&Program> = Vec::new();
        //             ast_index = i;
        //             while ast[ast_index] != Program::Expression(ExpressionType::Semicolon) {
        //                 statement.push(&ast[ast_index]);
        //                 ast_index += 1;
        //             }
        //             statement.reverse();
        //             for i in statement {
        //                 match i {
        //                     //FIXME: Operators are reversed in assembly on account of the backwards iteration through the ast.  This can cause unexpected results due to order of operations
        //                     Program::Expression(ExpressionType::Constant(num)) => write!(file_string, "\n\tmovl    ${}, %eax", num).unwrap(),
        //                     Program::Expression(ExpressionType::Negation) => file_string.push_str("\n\tneg    %eax"),
        //                     Program::Expression(ExpressionType::BitwiseComplement) => file_string.push_str("\n\tnot    %eax"),
        //                     Program::Expression(ExpressionType::LogicalNegation) => file_string.push_str("\n\tcmpl    $0, %eax\
        //                                                                                                            \n\tmovl    $0, %eax\
        //                                                                                                            \n\tsete    %al"),
        //                     //FIXME: OhGod order of operations ohagot oghot
        //                     //Program::Expression(ExpressionType::Addition) => file
        //                     Program::Statement(StatementType::Return) => file_string.push_str("\n\tret\n"),
        //                     _ => (),
        //                 }
        //             }
        //         }
        //         _ => (),
        //     }
        // }
        println!("{}", file_string);
        fs::write("/home/Dspivey/Programming/rust_projects/c_compiler/asm.s", file_string).expect("could not generate assembly file!!");
    }
}