use crate::command::Command;
use crate::context::Context;
// use crate::display_style::DisplayStyle;
use crate::config::{display_style, step_limit, DisplayStyle};
use crate::evaluate::EvalSteps;
use crate::expression::display::ecmascript::ECMAScriptStyle as ExprECMAScriptStyle;
use crate::expression::display::lazy_k::LazyKStyle as ExprLazyKStyle;
use crate::function::display::ecmascript::ECMAScriptStyle as FuncECMAScriptStyle;
use crate::function::display::lazy_k::LazyKStyle as FuncLazyKStyle;

pub struct Engine {
    context: Context,
    display_style: DisplayStyle,
}

impl Engine {
    pub fn new(context: Context) -> Self {
        Self {
            context,
            display_style: display_style(),
        }
    }

    pub fn run(&mut self, command: Command) {
        match command {
            Command::Del(i) => {
                self.context.del(&i);
            }

            Command::Update(f) => {
                self.context.def(f);
            }

            Command::Eval(e) => {
                let steps = EvalSteps::new(e, &self.context);
                for e in steps.take(step_limit()) {
                    match &self.display_style {
                        DisplayStyle::LazyK => println!("→ {}", ExprLazyKStyle(&e)),
                        DisplayStyle::Ecmascript => println!("→ {}", ExprECMAScriptStyle(&e)),
                    }
                }
            }

            Command::EvalLast(e) => {
                match &self.display_style {
                    DisplayStyle::LazyK => println!("{}", ExprLazyKStyle(&e)),
                    DisplayStyle::Ecmascript => println!("{}", ExprECMAScriptStyle(&e)),
                }

                let mut steps = EvalSteps::new(e, &self.context);
                if let (Some(e), _continue) = steps.eval_last(100) {
                    println!("→ ...");
                    match &self.display_style {
                        DisplayStyle::LazyK => println!("→ {}", ExprLazyKStyle(&e)),
                        DisplayStyle::Ecmascript => println!("→ {}", ExprECMAScriptStyle(&e)),
                    }
                } else {
                    // TODO
                }
            }

            Command::Info(i) => match self.context.get(&i) {
                Some(f) => match &self.display_style {
                    DisplayStyle::LazyK => println!("{}", FuncLazyKStyle(&f)),
                    DisplayStyle::Ecmascript => println!("{}", FuncECMAScriptStyle(&f)),
                },

                None => println!("{0} = {0}", i),
            },

            Command::Global => {
                self.context.for_each(|_i, f| println!("{}", f));
            }

            Command::Unlambda(e) => {
                match &self.display_style {
                    DisplayStyle::LazyK => println!("{}", ExprLazyKStyle(&e)),
                    DisplayStyle::Ecmascript => println!("{}", ExprECMAScriptStyle(&e)),
                }
                println!("== {}", e.unlambda());
            }

            _ => panic!("not implemented"),
        }
    }
}
