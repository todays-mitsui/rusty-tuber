mod expr;

#[macro_use]
extern crate combine;

use combine::EasyParser;
use expr::parser::expr;
use expr::Expr;

fn main() {
    println!(
        "{:?}",
        expr().easy_parse("` ^p.^x.^y.x `` ``s``s`ks``s`kk``s`ks``s`k`sik`kk :A :B")
    );

    let mut s:Expr = expr().easy_parse("^x.^y.^z.``xz`yz").unwrap().0;
    println!("{:?}", s);
}
