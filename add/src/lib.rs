#[allow(warnings)]
mod bindings;

use bindings::exports::docs::adder::add::Guest;

struct Component;

impl Guest for Component {
    fn add(lhs: i32, rhs: i32) -> i32 {
        lhs + rhs
    }
}

bindings::export!(Component with_types_in bindings);
