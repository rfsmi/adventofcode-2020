use clap::Parser;

mod utils;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(value_enum, default_value_t=Task::Latest)]
    task: Task,
}

utils::make_runner!(
    22+,
    23+,
    24+,
    25,
    21,
);

fn main() {
    run(Args::parse());
}
