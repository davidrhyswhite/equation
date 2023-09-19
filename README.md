# Equation

[![CI](https://github.com/davidrhyswhite/equation/actions/workflows/ci.yaml/badge.svg)](https://github.com/davidrhyswhite/equation/actions/workflows/ci.yaml)

A Rust library for mathematical expression evaluation and simplification.

### TODO

- [x] Rewrite pest parsing as PrattParser;
- [x] Modulo `%` & exponent `**` support;
- [x] Unary minus operator support;
- [ ] Support fractions as `f32`;
- [ ] Support basic triggernomic ratio functions sine, cosine & tangent as `sin`, `cos` & `tan`;
- [ ] Support evaluate `steps` parameter to break evaluation steps into a `Vec<&str>`;
- [ ] Support simplification & evaluation of algebraic expressions;

## Getting started

Add the following lines to your `Cargo.toml` dependencies:

```toml
[dependencies]
equation = "0.3.0"
```

## Examples

Evaluate basic arithmetic equations and return a `i32`.

```rust
use equation::evaluate;

fn main() {
    let result = evaluate("(1 + 2) * (1 + 2)"); // Returns Ok(9)
    match result {
        Ok(val) => println!("{}", val),
        Err(e) => println!("Error: {:?}", e),
    }
}
```

### Support

**Basic arithematic**

```rust
evaluate("(1 + 2) * 1 + 2"); // Returns Ok(5)
evaluate("4 * 3");           // Returns Ok(12)
evaluate("8 / 2");           // Returns Ok(4)
evaluate("16 - 4");          // Returns Ok(12)
```

**Negative calculations**

Unary operator. Returns the negation of its operand.

```rust
evaluate("-4 * 3"); // Returns Ok(-12)
evaluate("-8 / 2"); // Returns Ok(-4)
evaluate("-(4 + 4)"); // Returns Ok(-8)
```

**Exponentation**

Calculates the base to the exponent power, `base ^ exponent`.

```rust
evaluate("2 ^ 8");  // Returns Ok(256)
evaluate("2 exp 9"); // Returns Ok(512)
```

**Modulus**

Returns the integer remainder of dividing the two operands.	

```rust
evaluate("10 % 2");   // Returns Ok(0)
evaluate("10 % 4");   // Returns Ok(2)
evaluate("10 mod 3"); // Returns Ok(1)

            
            
```

## License

Licensed under MIT license ([LICENSE-MIT](LICENSE-MIT) | https://opensource.org/licenses/MIT) or under the Apache 2.0 ([LICENSE-APACHE](LICENSE-APACHE) | https://opensource.org/license/apache-2-0/).