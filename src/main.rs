mod expr;

#[macro_use]
extern crate combine;

use combine::EasyParser;
use expr::parser::expr;

fn main() {
    println!(
        "{:?}",
        expr().easy_parse("` ^p.^x.^y.x `` ``s``s`ks``s`kk``s`ks``s`k`sik`kk :A :B")
    );
}
