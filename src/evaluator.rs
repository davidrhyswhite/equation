use crate::parser::{parse_expression, EquationRule as Rule, ExpressionParser};

use pest::Parser;

/// Evaluates an arithmetic expression represented as a string.
///
/// # Examples
///
/// ```
/// use equation::evaluate;
/// assert_eq!(evaluate("(1 + 2) * 1 + 2"), Ok(5.0));
/// ```
///
/// # Arguments
///
/// * `expression` - A string slice that holds the arithmetic expression.
///
/// # Returns
///
/// * `Result<f64, pest::error::Error<Rule>>` - If the expression is valid, it returns a `Result::Ok(f64)`
///   containing the result of the evaluation. Otherwise, it returns a `Result::Err` with the parsing error.
// pub fn evaluate(expression: &str) -> Result<f64, pest::error::Error<Rule>> {
//     let pair = ExprParser::parse(Rule::equation, expression)?.next().unwrap();
//     Ok(parse_expression(pair))
// }
pub fn evaluate(expression: &str) -> Result<f64, pest::error::Error<Rule>> {
    let mut pairs = ExpressionParser::parse(Rule::equation, expression)?;
    Ok(parse_expression(pairs.next().unwrap().into_inner()))
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;

    fn set_test_cases() -> HashMap<&'static str, f64> {
        let test_cases = HashMap::from([
            // Basic arithmetic
            ("1 + 2 * 3", 7.0),
            ("6 + (1 + 2) * (2 + 1) + 1", 16.0),
            ("6 + 1 + 2 * 2 + 1 + 1", 13.0),
            // Negative numbers
            ("-1 + 2", 1.0),
            ("4 * -2", -8.0),
            // Exponents
            ("2 exp 10", 1024.0),
            ("2 ^ 6", 64.0),
            // Modulus
            ("10 % 2", 0.0),
            ("10 % 4", 2.0),
            ("10 mod 3", 1.0),
            // Triggernomic functions
            ("sin(45)", 0.8509035245341184),
            ("tan(30 + 15)", 1.6197751905438615),
            ("cos(90) + sin(45)", 0.40282990840494826),
        ]);

        test_cases
    }

    #[test]
    fn it_works_with_test_cases() {
        for test_case in set_test_cases() {
            let result = evaluate(&test_case.0);
            assert_eq!(
                result,
                Ok(test_case.1),
                "testing equation {} expected {}",
                &test_case.0,
                test_case.1
            );
        }
    }
}
