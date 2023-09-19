/// Represents a collection of trigonometric and hyperbolic functions.
///
/// This enumeration lists a set of supported mathematical functions. Each variant corresponds
/// to a specific function and can be executed with a given argument to produce a result.
#[derive(Debug)]
pub enum Functions {
    Sin,
    Cos,
    Tan,
    Asin,
    Acos,
    Atan,
    Sinh,
    Cosh,
    Tanh,
    Asinh,
    Acosh,
    Atanh,
}

impl Functions {
    /// Evaluates the trigonometric or hyperbolic function with the given argument.
    ///
    /// This method applies the function represented by the enum variant to the provided argument.
    ///
    /// # Arguments
    ///
    /// * `arg` - A floating-point value representing the input to the function.
    ///
    /// # Returns
    ///
    /// Returns the result of the function evaluation as a floating-point value.
    pub fn run(&self, arg: f64) -> f64 {
        match self {
            Self::Sin => arg.sin(),
            Self::Cos => arg.cos(),
            Self::Tan => arg.tan(),
            Self::Asin => arg.asin(),
            Self::Acos => arg.acos(),
            Self::Atan => arg.atan(),
            Self::Sinh => arg.sinh(),
            Self::Cosh => arg.cosh(),
            Self::Tanh => arg.tanh(),
            Self::Asinh => arg.asinh(),
            Self::Acosh => arg.acosh(),
            Self::Atanh => arg.atanh(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works_with_sin() {
        let func = Functions::Sin;
        assert_eq!(func.run(10.0), -0.5440211108893698);
    }

    #[test]
    fn it_works_with_cos() {
        let func = Functions::Cos;
        assert_eq!(func.run(10.0), -0.8390715290764524);
    }

    #[test]
    fn it_works_with_tan() {
        let func = Functions::Tan;
        assert_eq!(func.run(10.0), 0.6483608274590867);
    }

    #[test]
    fn it_works_with_asin() {
        let func = Functions::Asin;
        assert_eq!(func.run(0.5), 0.5235987755982988);
    }

    #[test]
    fn it_works_with_acos() {
        let func = Functions::Acos;
        assert_eq!(func.run(0.5), 1.0471975511965976);
    }

    #[test]
    fn it_works_with_atan() {
        let func = Functions::Atan;
        assert_eq!(func.run(10.0), 1.4711276743037345);
    }

    #[test]
    fn it_works_with_sinh() {
        let func = Functions::Sinh;
        assert_eq!(func.run(10.0), 11013.232874703393);
    }

    #[test]
    fn it_works_with_cosh() {
        let func = Functions::Cosh;
        assert_eq!(func.run(10.0), 11013.232920103323);
    }

    #[test]
    fn it_works_with_tanh() {
        let func = Functions::Tanh;
        assert_eq!(func.run(10.0), 0.9999999958776927);
    }

    #[test]
    fn it_works_with_asinh() {
        let func = Functions::Asinh;
        assert_eq!(func.run(10.0), 2.99822295029797);
    }

    #[test]
    fn it_works_with_acosh() {
        let func = Functions::Acosh;
        assert_eq!(func.run(10.0), 2.993222846126381);
    }

    #[test]
    fn it_works_with_atanh() {
        let func = Functions::Atanh;
        assert_eq!(func.run(0.5), 0.5493061443340549);
    }
}
