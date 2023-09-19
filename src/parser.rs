use crate::functions::Functions;
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
        .op(Op::infix(Rule::multiply, Assoc::Left)
            | Op::infix(Rule::divide, Assoc::Left)
            | Op::infix(Rule::exponentiation, Assoc::Left)
            | Op::infix(Rule::modulo, Assoc::Left))
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
fn primary_parser(primary: Pair<Rule>) -> f64 {
    match primary.as_rule() {
        Rule::integer => primary.as_str().parse::<f64>().unwrap(),
        Rule::expression => parse_expression(primary.into_inner()),
        Rule::function_call => parse_function_call(primary.into_inner()),
        rule => unreachable!("parse expected atom, found {:?}", rule),
    }
}

/// Parses and evaluates a trigonometric function call based on the given rule pairs.
///
/// This function processes pairs representing a function call where the first pair
/// is expected to be the function name (e.g., "sin", "cos", "tan") and the subsequent
/// pairs represent the argument(s) to the function. The function argument is expected
/// to be an angle in degrees, and the function returns the integer result of the
/// trigonometric calculation.
///
/// # Arguments
///
/// * `pairs` - A mutable iterator of `Pairs<Rule>` where the first pair represents
///   the function name and the subsequent pairs represent the function's argument(s).
///
/// # Returns
///
/// Returns the result of the trigonometric function call as an `f64`.
/// The return value is truncated from a floating point representation.
///
/// # Panics
///
/// The function will panic if:
///
/// * The first pair does not represent a recognized function name.
/// * There are unexpected or missing pairs for the function call.
fn parse_function_call(mut pairs: Pairs<Rule>) -> f64 {
    // First pair is the function name, second pair is the expression
    let func_name = pairs.next().unwrap();
    let arg_value = parse_expression(pairs);

    match func_name.as_rule() {
        Rule::function_name => match func_name.as_str() {
            "sin" => Functions::Sin.run(arg_value),
            "cos" => Functions::Cos.run(arg_value),
            "tan" => Functions::Tan.run(arg_value),
            "asin" => Functions::Asin.run(arg_value),
            "acos" => Functions::Acos.run(arg_value),
            "atan" => Functions::Atan.run(arg_value),
            "sinh" => Functions::Sinh.run(arg_value),
            "cosh" => Functions::Cosh.run(arg_value),
            "tanh" => Functions::Tanh.run(arg_value),
            "asinh" => Functions::Asinh.run(arg_value),
            "acosh" => Functions::Acosh.run(arg_value),
            "atanh" => Functions::Atanh.run(arg_value),
            _ => unreachable!(),
        },
        _ => unreachable!(),
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
/// Returns the result of the arithmetic operation as an `f64`.
///
/// # Panics
///
/// This function will panic if the provided `op` does not match a recognized arithmetic operation.
fn infix_parser(lhs: f64, op: Pair<Rule>, rhs: f64) -> f64 {
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
/// Returns the result of the prefix operation as an `f64`.
///
/// # Panics
///
/// This function will panic if the provided `op` does not match a recognized prefix operation.
fn prefix_parser(op: Pair<Rule>, rhs: f64) -> f64 {
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
/// * `f64` - The result of evaluating the expression.
pub fn parse_expression(pairs: Pairs<Rule>) -> f64 {
    retrieve_parser()
        .map_primary(primary_parser)
        .map_infix(infix_parser)
        .map_prefix(prefix_parser)
        .parse(pairs)
}
