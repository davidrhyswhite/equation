#[macro_use]
extern crate pest_derive;

use pest::Parser;

/// The `ExprParser` struct provides parsing capability for arithmetic expressions.
/// It uses the grammar defined in `grammar.pest`.
#[derive(Parser)]
#[grammar = "grammar.pest"]
struct ExprParser;

/// Evaluates an arithmetic expression represented as a string.
///
/// # Examples
///
/// ```
/// use equation::evaluate;
/// assert_eq!(evaluate("(1 + 2) * 1 + 2"), Ok(5));
/// ```
///
/// # Arguments
///
/// * `expression` - A string slice that holds the arithmetic expression.
///
/// # Returns
///
/// * `Result<i32, pest::error::Error<Rule>>` - If the expression is valid, it returns a `Result::Ok(i32)`
///   containing the result of the evaluation. Otherwise, it returns a `Result::Err` with the parsing error.
pub fn evaluate(expression: &str) -> Result<i32, pest::error::Error<Rule>> {
    let pair = ExprParser::parse(Rule::equation, expression)?.next().unwrap();
    Ok(evaluate_expression(pair))
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
fn evaluate_expression(pair: pest::iterators::Pair<Rule>) -> i32 {
    match pair.as_rule() {
        Rule::integer => pair.as_str().parse::<i32>().unwrap(),
        Rule::expr => {
            let mut inner = pair.into_inner();
            let mut acc = evaluate_expression(inner.next().unwrap());
            while let Some(op_pair) = inner.next() {
                let op = op_pair.as_rule();
                let next_val = evaluate_expression(inner.next().unwrap());
                match op {
                    Rule::add => acc += next_val,
                    Rule::subtract => acc -= next_val,
                    _ => unreachable!(),
                }
            }
            acc
        },
        Rule::term => {
            let mut inner = pair.into_inner();
            let mut acc = evaluate_expression(inner.next().unwrap());
            while let Some(op_pair) = inner.next() {
                let op = op_pair.as_rule();
                let next_val = evaluate_expression(inner.next().unwrap());
                match op {
                    Rule::multiply => acc *= next_val,
                    Rule::divide => acc /= next_val,
                    _ => unreachable!(),
                }
            }
            acc
        }
        Rule::primary => {
            let inner = pair.into_inner().next().unwrap();
            evaluate_expression(inner)
        }
        _ => unreachable!(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works_with_basic_arithmetic() {
        let result = evaluate("1 + 2 * 3");

        assert_eq!(result, Ok(7));
    }

    #[test]
    fn it_works_with_parentheses() {
        let result = evaluate("(1 + 2) * 3");

        assert_eq!(result, Ok(9));
    }

    #[test]
    fn it_works_with_ordering_equations() {
        let result = evaluate("6 + (1 + 2) * (1 + 2) + 1");

        assert_eq!(result, Ok(16));
    }
}
