use clap::Parser;
mod day1;
mod day2;
mod day3;
mod day4;
mod day5;
mod day6;
mod day7;

#[derive(Debug, Parser)]
struct Options {
    #[arg(short, long)]
    slow: bool,
}

fn main() {
    let opts = Options::parse();

    day1::main();
    day2::main();
    day3::main();
    day4::main();
    day5::main();
    if opts.slow {
        day6::main();
        day7::main();
    }
}
