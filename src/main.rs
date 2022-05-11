use calculator::eval_expression;

fn main() {
    println!("=== SIMPLE CALCULATOR ===\n");
    let expr = "5 * 5";
    println!("{}", eval_expression(expr));
}
