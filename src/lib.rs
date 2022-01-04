mod parser;

use bitvec::prelude::*;

use crate::parser::parse_program;

#[derive(Debug)]
pub enum ASTreeNode {
    Program(Vec<ASTreeNode>),
    Statement(Vec<ASTreeNode>),
    Expression(Vec<ASTreeNode>),
    Operation(Vec<ASTreeNode>),
    Literal(bool),
    LiteralArr(BitVec),
    GeneratedArr(BitVec),
    Variable(String),
    VariableDef(String, Box<ASTreeNode>),
    FunctionCall(String, Vec<ASTreeNode>),
    FunctionDef(String, Vec<ASTreeNode>, Vec<ASTreeNode>),
    FunctionArgs(Vec<ASTreeNode>),
}

#[derive(Debug)]
pub struct ASTree {
    program: ASTreeNode,
}

#[derive(Debug)]
pub enum BLError {
    GeneralError,
    ParserError(String),
}

pub fn compile(input: &str) -> Result<ASTree, BLError> {
    let (input, program) = parse_program(input).map_err(|e| match e {
        nom::Err::Error(f) | nom::Err::Failure(f) => {
            let line = f.input.lines().next();
            match line {
                Some(s) => BLError::ParserError(s.to_string()),
                None => BLError::GeneralError
            }
        },
        _ => BLError::GeneralError
    })?;
    let program = ASTree { program };

    Ok(program)
}

pub fn run(ast: ASTree) -> Result<(), BLError> {
    println!("{:#?}", ast.program);
    Ok(())
}