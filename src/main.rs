mod input;
mod q1;
mod q2;
mod q3;
mod q4;

use clap::Parser;

#[derive(Parser)]
struct Cli {
    #[clap(short, long)]
    question: Question,
}

fn main() {
    let args = Cli::parse();
    (args.question.get())();
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, clap::ValueEnum)]
enum Question {
    Q1,
    Q2,
    Q3,
    Q4,
}

impl Question {
    fn get(&self) -> fn() {
        match self {
            Question::Q1 => q1::q1,
            Question::Q2 => q2::q2,
            Question::Q3 => q3::q3,
            Question::Q4 => q4::q4,
        }
    }
}
