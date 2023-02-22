use clap::Parser;

mod utils;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(value_enum, default_value_t=Task::Latest)]
    task: Task,
}

utils::make_runner!(
    24+,
    23+,
    22+,
    25,
);

fn main() {
    run(Args::parse());
}
