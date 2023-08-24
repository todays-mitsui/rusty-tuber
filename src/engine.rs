use crate::command::Command;
use crate::context::Context;
use crate::evaluate::EvalSteps;

pub struct Engine {
    context: Context,
}

impl Engine {
    pub fn new(context: Context) -> Self {
        Self { context }
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
                println!("{}", e);

                let steps = EvalSteps::new(e, &self.context);
                for e in steps.take(100) {
                    println!("→ {}", e);
                }
            }

            Command::Info(i) => match self.context.get(&i) {
                // TODO: ちゃんとする
                Some(f) => println!("{}", Command::Update(f.clone())),
                None => println!("{} = {}", i, i),
            },

            Command::Unlambda(e) => {
                println!("{}", e);
                println!("== {}", e.unlambda());
            }

            _ => panic!("not implemented"),
        }
    }
}
