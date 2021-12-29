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
}

#[derive(Debug)]
pub struct ASTree {
    program: ASTreeNode,
}

#[derive(Debug)]
pub enum BLError {
    GeneralError,
}

pub fn compile(input: &str) -> Result<ASTree, BLError> {
    let (_, program) = parse_program(input).map_err(|_| BLError::GeneralError)?;
    let program = ASTree { program };

    Ok(program)
}

pub fn run(ast: ASTree) -> Result<(), BLError> {
    println!("{:#?}", ast.program);
    Ok(())
}