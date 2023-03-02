mod expr;

#[macro_use]
extern crate combine;

use combine::Parser;

fn main() {
    println!("{:?}", expr::parser::var().parse("_"));
}
