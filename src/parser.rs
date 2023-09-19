use crate::operation::Operation;

use pest::{
    iterators::{Pair, Pairs},
    pratt_parser::{Assoc, Op, PrattParser},
};

/// Represents a parser derived from the grammar specified in "grammar.pest".
///
/// This parser is used to parse mathematical expressions based on the rules defined
/// in the external grammar file. The parsing rules are determined by the provided grammar.
#[derive(Parser)]
#[grammar = "grammar.pest"]
pub struct ExpressionParser;

/// A type alias for the set of rules defined in the grammar.
///
/// This type makes it easier to refer to the specific rules within the grammar, enhancing code clarity.
pub type EquationRule = Rule;

/// Constructs and returns a `PrattParser` tailored for parsing arithmetic expressions.
///
/// The parser is configured to recognize addition, subtraction, multiplication, and division
/// as left-associative operations based on the rules specified in "grammar.pest".
///
/// # Returns
///
/// Returns a `PrattParser<Rule>` instance with pre-configured operations based on the provided grammar.
fn retrieve_parser() -> PrattParser<Rule> {
    PrattParser::new()
        // Define addition and subtraction as left associative operations.
        .op(Op::infix(Rule::add, Assoc::Left) | Op::infix(Rule::subtract, Assoc::Left))
        // Define multiplication and division as left associative operations.
        .op(
            Op::infix(Rule::multiply, Assoc::Left) |
                Op::infix(Rule::divide, Assoc::Left) |
                Op::infix(Rule::exponentiation, Assoc::Left) |
                Op::infix(Rule::modulo, Assoc::Left)
            )
        // Define the unary minus as a prefix operation
        .op(Op::prefix(Rule::unary_minus))
}

/// Evaluates the primary component of an arithmetic expression.
///
/// This function checks the rule associated with a primary component of the arithmetic expression
/// and evaluates it accordingly. For instance, if the rule represents an integer, it converts the
/// string representation to an actual integer. If the rule represents another nested expression, it
/// recursively evaluates the nested expression.
///
/// # Arguments
///
/// * `primary` - A `Pair<Rule>` representing a primary component of the arithmetic expression.
///
/// # Returns
///
/// Returns the integer result of evaluating the primary component.
///
/// # Panics
///
/// This function will panic if an unrecognized rule is encountered, indicating an unexpected token in the expression.
fn primary_parser(primary: Pair<Rule>) -> i32 {
    match primary.as_rule() {
        Rule::integer => primary.as_str().parse::<i32>().unwrap(),
        Rule::expression => parse_expression(primary.into_inner()),
        rule => unreachable!("parse expected atom, found {:?}", rule),
    }
}

/// Evaluates an infix arithmetic operation given the left-hand side value (`lhs`),
/// the operation itself (`op`), and the right-hand side value (`rhs`).
///
/// This function matches against known operations (`add`, `subtract`, `multiply`, `divide`)
/// and applies the corresponding arithmetic operation. If an unrecognized operation is encountered,
/// the function will panic with an "unreachable" message.
///
/// # Arguments
///
/// * `lhs` - The left-hand side operand of the arithmetic expression.
/// * `op` - The infix arithmetic operation to be applied.
/// * `rhs` - The right-hand side operand of the arithmetic expression.
///
/// # Returns
///
/// Returns the result of the arithmetic operation as an `i32`.
///
/// # Panics
///
/// This function will panic if the provided `op` does not match a recognized arithmetic operation.
fn infix_parser(lhs: i32, op: Pair<Rule>, rhs: i32) -> i32 {
    match op.as_rule() {
        Rule::add => Operation::Add.run(lhs, rhs),
        Rule::subtract => Operation::Subtract.run(lhs, rhs),
        Rule::multiply => Operation::Multiply.run(lhs, rhs),
        Rule::divide => Operation::Divide.run(lhs, rhs),
        Rule::exponentiation => Operation::Exponent.run(lhs, rhs),
        Rule::modulo => Operation::Modulo.run(lhs, rhs),
        rule => unreachable!("parser expected infix operation, found {:?}", rule),
    }
}
/// Evaluates a prefix operation on an arithmetic value.
///
/// This function processes recognized unary prefix operations on the provided right-hand side value (`rhs`).
/// Currently, it handles unary negation (`unary_minus`). If an unrecognized operation is encountered,
/// the function will panic.
///
/// # Arguments
///
/// * `op` - A `Pair<Rule>` representing the prefix operation to be applied.
/// * `rhs` - The right-hand side operand of the arithmetic expression. In the context of prefix operations,
///           it's the value the operation is applied to.
///
/// # Returns
///
/// Returns the result of the prefix operation as an `i32`.
///
/// # Panics
///
/// This function will panic if the provided `op` does not match a recognized prefix operation.
fn prefix_parser(op: Pair<Rule>, rhs: i32) -> i32 {
    match op.as_rule() {
        Rule::unary_minus => -rhs,
        _ => unreachable!(),
    }
}

/// Recursively evaluates an expression provided as a `Pair` of `Rule`.
///
/// This function handles different arithmetic operations and their precedence according to the provided grammar.
///
/// # Arguments
///
/// * `pair` - A `Pair<Rule>` representing part or all of an arithmetic expression.
///
/// # Returns
///
/// * `i32` - The result of evaluating the expression.
pub fn parse_expression(pairs: Pairs<Rule>) -> i32 {
    retrieve_parser()
        .map_primary(primary_parser)
        .map_infix(infix_parser)
        .map_prefix(prefix_parser)
        .parse(pairs)
}
