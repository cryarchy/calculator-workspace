#[allow(warnings)]
mod bindings;

use bindings::docs::adder::add::add;
use bindings::exports::component::calculator::calculate::Guest;

struct Component;

impl Guest for Component {
    fn eval_expression(expr: String) -> i32 {
        let parts = expr.split_whitespace().collect::<Vec<_>>();
        if parts.len() != 3 {
            panic!("Invalid expression");
        }
        let lhs = parts[0].parse::<i32>().expect("Invalid number");
        let rhs = parts[2].parse::<i32>().expect("Invalid number");
        match parts[1] {
            "+" => add(lhs, rhs),
            "-" => lhs - rhs,
            "*" => lhs * rhs,
            "/" => lhs / rhs,
            _ => panic!("Invalid operator"),
        }
    }
}

bindings::export!(Component with_types_in bindings);
