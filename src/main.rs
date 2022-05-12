use calculator::eval_expression;

fn parse_input() -> String {
    let mut line = String::new();
    std::io::stdin()
        .read_line(&mut line)
        .expect("Must enter an expression");
    let line = line.trim();
    line.to_string()
}

fn main() {
    println!("=== SIMPLE CALCULATOR ===\n");
    loop {
        let expr = parse_input();
        if expr.as_str() == "q" {
            break;
        }
        let expr = expr.as_str();
        println!("{}\n", eval_expression(expr));
    }
}
