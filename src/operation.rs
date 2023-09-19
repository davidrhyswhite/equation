/// Represents the primary arithmetic operations.
///
/// This enumeration defines the basic arithmetic operations that can be performed.
/// Each variant corresponds to one of the fundamental mathematical operations.
#[derive(Debug)]
pub enum Operation {
    Add,
    Subtract,
    Multiply,
    Divide,
    Exponent,
    Modulo,
}

impl Operation {
    /// Executes the arithmetic operation using the provided operands.
    ///
    /// The operation to be executed is determined by the variant on which this method
    /// is invoked. This method provides a convenient way to evaluate an arithmetic operation
    /// without explicitly matching against each variant.
    ///
    /// # Arguments
    ///
    /// * `lhs` - The left-hand side operand.
    /// * `rhs` - The right-hand side operand.
    ///
    /// # Returns
    ///
    /// Returns the result of the arithmetic operation as an `i32`.
    pub fn run(&self, lhs: i32, rhs: i32) -> i32 {
        match self {
            Self::Add => lhs + rhs,
            Self::Subtract => lhs - rhs,
            Self::Multiply => lhs * rhs,
            Self::Divide => lhs / rhs,
            Self::Exponent => lhs.pow(rhs.try_into().unwrap()),
            Self::Modulo => lhs % rhs,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works_with_addition() {
        let operation = Operation::Add;
        assert_eq!(operation.run(10, 2), 12);
    }

    #[test]
    fn it_works_with_subtraction() {
        let operation = Operation::Subtract;
        assert_eq!(operation.run(10, 2), 8);
    }

    #[test]
    fn it_works_with_multiplication() {
        let operation = Operation::Multiply;
        assert_eq!(operation.run(10, 2), 20);
    }

    #[test]
    fn it_works_with_division() {
        let operation = Operation::Divide;
        assert_eq!(operation.run(10, 2), 5);
    }
}
