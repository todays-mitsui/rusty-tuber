use crate::command::Command;
use crate::environment::Env;
use crate::evaluate::EvalSteps;

pub struct Engine {
    env: Env,
}

impl Engine {
    pub fn new(env: Env) -> Self {
        Engine { env }
    }

    pub fn run(&mut self, command: Command) {
        match command {
            Command::Del(i) => {
                self.env.del(&i);
            }

            Command::Update(i, f) => {
                self.env.def(i, f);
            }

            Command::Eval(e) => {
                println!("{}", e);

                let steps = EvalSteps::new(e, &self.env);
                for e in steps.take(100) {
                    println!("→ {}", e);
                }
            }

            Command::Info(i) => match self.env.get(&i) {
                // TODO: ちゃんとする
                Some(f) => println!("{}", Command::Update(i, f.clone())),
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
