use bitvec::prelude::*;
use nom::{
    IResult,
    bytes::complete::{tag},
    character::complete::{multispace0, multispace1, line_ending, one_of, digit1, alphanumeric1, alphanumeric0, alpha1},
    multi::{many0, many1, separated_list1},
    branch::{alt},
    sequence::{terminated, delimited, pair}, combinator::recognize,
};

use crate::ASTreeNode;

fn parse_literal(input: &str) -> IResult<&str, ASTreeNode> {
    let (input, output) = one_of("01")(input)?;

    let output = match output {
        '0' => false,
        '1' => true,
        _ => {unreachable!()}
    };

    Ok((input, ASTreeNode::Literal(output)))
}

fn parse_literal_arr(input: &str) -> IResult<&str, ASTreeNode> {
    let (input, _) = tag("{")(input)?;
    let (input, _) = multispace0(input)?; // optional whitespace
    let (input, output_arr) = separated_list1(terminated(tag(","), multispace0),
                                              parse_literal)(input)?;
    let (input, _) = multispace0(input)?; // optional whitespace
    let (input, _) = tag("}")(input)?;

    let mut output = BitVec::new();
    output.reserve(output_arr.len());
    for node in output_arr {
        if let ASTreeNode::Literal(i) = node {
            output.push(i);
        }
    }

    Ok((input, ASTreeNode::LiteralArr(output)))
}

fn parse_generated_arr(input: &str) -> IResult<&str, ASTreeNode> {
    let (input, _) = tag("{")(input)?;
    let (input, _) = multispace0(input)?; // optional whitespace
    let (input, _) = tag(":")(input)?;
    let (input, output) = digit1(input)?;
    let (input, _) = multispace0(input)?; // optional whitespace
    let (input, _) = tag("}")(input)?;

    let arr_size: usize = output.parse().unwrap();
    let mut output = BitVec::new();
    output.resize(arr_size, false);

    Ok((input, ASTreeNode::GeneratedArr(output)))
}

fn parse_expression(input: &str) -> IResult<&str, ASTreeNode> {
    let (input, output) = alt((parse_literal, parse_literal_arr, parse_generated_arr, parse_variable))(input)?;

    Ok((input, output))
}

fn parse_variable_helper(input: &str) -> IResult<&str, String> {
    let (input, output) = recognize(pair(alt((alpha1, tag("_"))), many0(alt((alphanumeric1, tag("_"))))))(input)?;

    Ok((input, String::from(output)))
}

fn parse_variable(input: &str) -> IResult<&str, ASTreeNode> {
    let (input, output) = parse_variable_helper(input)?;

    Ok((input, ASTreeNode::Variable(output)))
}

fn parse_var_definition(input: &str) -> IResult<&str, ASTreeNode> {
    let (input, _) = tag("let")(input)?;
    let (input, _) = multispace1(input)?;
    let (input, var_name) = parse_variable_helper(input)?;
    let (input, _) = multispace0(input)?;
    let (input, _) = tag("=")(input)?;
    let (input, _) = multispace0(input)?;
    let (input, var_val) = parse_expression(input)?;

    let var_val = Box::new(var_val);

    Ok((input, ASTreeNode::VariableDef(var_name, var_val)))
}

fn parse_statement(input: &str) -> IResult<&str, ASTreeNode> {
    let (input, output) = delimited(multispace0, alt((parse_var_definition, parse_expression)), multispace0)(input)?;
    let (input, _) = tag(";")(input)?;

    Ok((input, ASTreeNode::Statement(vec![output])))
}

pub fn parse_program(input: &str) -> IResult<&str, ASTreeNode> {
    let (input, output) = many1(parse_statement)(input)?;
    
    let program = ASTreeNode::Program(output);

    Ok((input, program))
}