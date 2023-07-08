use std::collections::HashMap;

use expr::{Expr, Identifier};
use func::Func;

type Env = HashMap<Identifier, Func>;

fn ality(env: &Env, id: &Identifier) -> Option<usize> {
    env.get(id).map(|f| f.ality())
}
