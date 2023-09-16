use crate::parser::{parse_expression, EquationRule as Rule, ExpressionParser};

use pest::Parser;

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
// pub fn evaluate(expression: &str) -> Result<i32, pest::error::Error<Rule>> {
//     let pair = ExprParser::parse(Rule::equation, expression)?.next().unwrap();
//     Ok(parse_expression(pair))
// }
pub fn evaluate(expression: &str) -> Result<i32, pest::error::Error<Rule>> {
    let mut pairs = ExpressionParser::parse(Rule::equation, expression)?;
    Ok(parse_expression(pairs.next().unwrap().into_inner()))
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
        let result = evaluate("6 + (1 + 2) * (2 + 1) + 1");
        assert_eq!(result, Ok(16));
    }

    #[test]
    fn it_works_with_ordering_equations_based_on_bodmas() {
        let result = evaluate("6 + 1 + 2 * 2 + 1 + 1");
        assert_eq!(result, Ok(13));
    }

    #[test]
    fn it_works_with_negative_numbers() {
        let result = evaluate("-1 + 2");
        assert_eq!(result, Ok(1));
    }

    #[test]
    fn it_works_with_more_negative_numbers() {
        let result = evaluate("4 * -2");
        assert_eq!(result, Ok(-8));
    }
}
