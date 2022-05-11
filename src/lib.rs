struct Expr {
    lhs: f32,
    rhs: f32,
    op: Operator
}

impl Expr {
    fn new(lhs: f32, rhs:f32, op: Operator) -> Self {
        Self {
            lhs,
            rhs,
            op
        }
    }

    fn eval(&self) -> f32 {
        use Operator::*;
        match self.op {
            PLUS => self.lhs + self.rhs,
            MINUS => self.lhs - self.rhs,
            MULT => self.lhs * self.rhs,
            DIV => {
                if self.rhs == 0.0 {
                    panic!("[Divide error]: Cannot divide by 0")
                }
                self.lhs / self.rhs
            }
        }
    }
}

enum Operator {
    PLUS,
    MINUS,
    MULT,
    DIV
}

pub fn eval_expression(expression: &str) -> f32 {
    let expr = parse_expression(&expression);
    expr.eval()
}
fn parse_expression(expression: &str) -> Expr {
    let expression = expression.split(" ");
    let expression:Vec<&str> = expression.collect();
    if expression.len() != 3 {
        panic!("[Expression error]: Invalid expression")
    }

    // parse LHS and RHS 
    let lhs = expression[0];
    let op  = expression[1];
    let rhs = expression[2];

    let lhs: f32 = match lhs.parse() {
        Ok(x) => x,
        Err(e) => panic!("[LHS error]: {}", e)
    };
    let rhs: f32 = match rhs.parse() {
        Ok(x) => x,
        Err(e) => panic!("[RHS error]: {}", e)
    };

    // parse operator
    let op = match op {
        "+" => Operator::PLUS,
        "-" => Operator::MINUS,
        "*" => Operator::MULT,
        "/" => Operator::DIV,
         _  => panic!("[Expression error]: Invalid operator")
    };

    Expr::new(lhs, rhs, op)
}


#[cfg(test)]

#[test]
fn can_add() {
    let expr = Expr::new(5.0, 5.0, Operator::PLUS);
    assert_eq!(expr.eval(), 10.0);
}

#[test]
fn can_sub() {
    let expr = Expr::new(5.0, 5.0, Operator::MINUS);
    assert_eq!(expr.eval(), 0.0);
}

#[test]
fn can_mult() {
    let expr = Expr::new(5.0, 5.0, Operator::MULT);
    assert_eq!(expr.eval(), 25.0);
}

#[test]
fn can_div() {
    let expr = Expr::new(5.0, 5.0, Operator::DIV);
    assert_eq!(expr.eval(), 1.0);
}

#[test]
#[should_panic]
fn panic_at_divide_by_zero() {
    let expr = Expr::new(5.0, 0.0, Operator::DIV);
    expr.eval();
}


#[test]
#[should_panic]
fn panic_at_invalid_lhs() {
    let expr = "foo + 1";
    parse_expression(expr);
}

#[test]
#[should_panic]
fn panic_at_invalid_rhs() {
    let expr = "1 + foo";
    parse_expression(expr);
}

#[test]
#[should_panic]
fn panic_at_invalid_op() {
    let expr = "1 x 1";
    parse_expression(expr);
}
