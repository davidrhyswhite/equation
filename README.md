# Equation

A Rust library for mathematical expression evaluation and simplification.

### TODO

- [x] Rewrite pest parsing as PrattParser;
- [ ] Modulo `%` & exponent `**` support;
- [ ] Unary minus operator support;
- [ ] Support fractions as `f32`;
- [ ] Support basic triggernomic ratio functions sine, cosine & tangent as `sin`, `cos` & `tan`;
- [ ] Support evaluate `steps` parameter to break evaluation steps into a `Vec<&str>`;
- [ ] Support simplification & evaluation of algebraic expressions;
- [ ] Create simplify 

## Getting started

Add the following lines to your `Cargo.toml` dependencies:

```toml
[dependencies]
equation = "0.1.0"
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

    evaluate("(1 + 2) * 1 + 2"); // Returns Ok(5)
}
```

## License

Licensed under MIT license ([LICENSE-MIT](LICENSE-MIT) | https://opensource.org/licenses/MIT) or under the Apache 2.0 ([LICENSE-APACHE](LICENSE-APACHE) | https://opensource.org/license/apache-2-0/).