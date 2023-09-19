# Equation

[![CI](https://github.com/davidrhyswhite/equation/actions/workflows/ci.yaml/badge.svg)](https://github.com/davidrhyswhite/equation/actions/workflows/ci.yaml.0)

A Rust library for mathematical expression evaluation and simplification.

### TODO

- [x] Rewrite pest parsing as PrattParser;
- [x] Modulo `%` & exponent `**` support;
- [x] Unary minus operator support;
- [x] Support fractions as ~~`f32`~~ `f64`;
- [x] Support basic triggernomic ratio functions sine, cosine & tangent as `sin`, `cos` & `tan`;
- [ ] Support mathematical constants `pi`, `e`, `tau`;
- [ ] Support functions with additional arguments, `atan2(y, x)`;
- [ ] Support evaluate `steps` parameter to break evaluation steps into a `Vec<&str>`;
- [ ] Support simplification & evaluation of algebraic expressions;
- [ ] Support for vectors and basic vector arithmetic & operations;

## Getting started

Add the following lines to your `Cargo.toml` dependencies:

```toml
[dependencies]
equation = "1.0.0"
```

## Examples

Evaluate basic arithmetic equations and return a `f64`.

```rust
use equation::evaluate;

fn main() {
    let result = evaluate("(1 + 2) * (1 + 2)"); // Returns Ok(9.0)
    match result {
        Ok(val) => println!("{}", val),
        Err(e) => println!("Error: {:?}", e),
    }
}
```

### Support

**Basic arithematic**

```rust
evaluate("(1 + 2) * 1 + 2"); // Returns Ok(5.0)
evaluate("4 * 3");           // Returns Ok(12.0)
evaluate("8 / 2");           // Returns Ok(4.0)
evaluate("16 - 4");          // Returns Ok(12.0)
```

**Negative calculations**

Unary operator. Returns the negation of its operand.

```rust
evaluate("-4 * 3"); // Returns Ok(-12.0)
evaluate("-8 / 2"); // Returns Ok(-4.0)
evaluate("-(4 + 4)"); // Returns Ok(-8.0)
```

**Exponentation**

Calculates the base to the exponent power, `base ^ exponent`.

```rust
evaluate("2 ^ 8");   // Returns Ok(256.0)
evaluate("2 exp 9"); // Returns Ok(512.0)
```

**Modulus**

Returns the integer remainder of dividing the two operands.	

```rust
evaluate("10 % 2");   // Returns Ok(0.0)
evaluate("10 % 4");   // Returns Ok(2.0)
evaluate("10 mod 3"); // Returns Ok(1.0)
```

**Triggernomic functions**


| Function      | Description                                | Example                   | Result                     |
| ------------- | ------------------------------------------ | ------------------------- | -------------------------- |
| sin           | Sine of the input                          | `evaluate("sin(10.0));`   | `Ok(-0.5440211108893698)`  |
| cos           | Cosine of the input                        | `evaluate("cos(10.0));`   | `Ok(-0.8390715290764524)`  |
| tan           | Tangent of the input                       | `evaluate("tan(10.0));`   | `Ok(0.6483608274590867)`   |
| asin          | Inverse sine of the input                  | `evaluate("asin(0.5));`   | `Ok(0.5235987755982988);)` |
| acos          | Inverse cosine of the input                | `evaluate("acos(0.5));`   | `Ok(1.0471975511965976);)` |
| atan          | Inverse tangent of the input               | `evaluate("atan(10.0));`  | `Ok(1.4711276743037345);)` |
| sinh          | Hyperbolic sine of the input               | `evaluate("sinh(10.0));`  | `Ok(11013.232874703393);)` |
| cosh          | Hyperbolic cosine of the input             | `evaluate("cosh(10.0));`  | `Ok(11013.232920103323);)` |
| tanh          | Hyperbolic tangent of the input            | `evaluate("tanh(10.0));`  | `Ok(0.9999999958776927);)` |
| asinh         | Inverse hyperbolic sine of the input       | `evaluate("asinh(10.0));` | `Ok(2.99822295029797);)`   |
| acosh         | Inverse hyperbolic cosine of the input     | `evaluate("acosh(10.0));` | `Ok(2.993222846126381);)`  |
| atanh         | Inverse hyperbolic tangent of the input    | `evaluate("atanh(10.0));` | `Ok(0.5493061443340549);)` |

## License

Licensed under MIT license ([LICENSE-MIT](LICENSE-MIT) | https://opensource.org/licenses/MIT) or under the Apache 2.0 ([LICENSE-APACHE](LICENSE-APACHE) | https://opensource.org/license/apache-2-0/).