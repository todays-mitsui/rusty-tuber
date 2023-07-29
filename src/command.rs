mod display;

use crate::expression::Expr;
use crate::function::Func;
use crate::identifier::Ident;

#[derive(Debug, PartialEq)]
pub enum Command {
    Del(Ident),            // 関数を削除
    Add(Ident, Func),      // 関数定義 (定義済み関数の上書きを許さない)
    Update(Ident, Func),   // 関数定義 (定義済み関数の上書きを許す)
    Eval(Expr),            // β変換列を表示
    EvalLast(Expr),        // β変結果のみ表示
    EvalHead(usize, Expr), // β変換列の先頭のみ表示
    EvalTail(usize, Expr), // β変換列の末尾のみ表示
    Info(Ident),           // Global から定義済み関数を検索
    Global,                // Global 全体を表示
}
