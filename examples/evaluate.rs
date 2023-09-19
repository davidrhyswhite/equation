use equation::evaluate;

fn example(expression: &str) {
    let result = evaluate(expression);
    match result {
        Ok(val) => println!("{} = {}", expression, val),
        Err(e) => println!("Error: {:?}", e),
    }
}

fn main() {
    example("1 + 2 * 3            ");
    example("(1 + 2) * 3          ");
    example("(1 + 2) * 1 + 2      ");
    example("(1 + 2) * (1 + 2)    ");
    example("6 + 1 + 2 * 2 + 1 + 1");
    example("4 * -3               ");
    example("-4 - -2              ");
    example("2 exp 10             ");
    example("2 ^ 6                ");
}
