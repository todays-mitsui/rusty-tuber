mod expr;

use combine::Parser;

fn main() {
    println!("{:?}", expr::parser::var().parse("_"));
}
