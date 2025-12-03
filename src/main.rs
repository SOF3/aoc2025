use std::{env, fs};

fn main() {
    let arg = env::args().nth(1).expect("Usage: cargo run <solution_id>");
    let func = aoc2025::select(&arg).expect("Unknown solution ID");

    let day = if arg.starts_with('d')
        && let Some(p) = arg.find('p')
    {
        &arg[1..p]
    } else {
        panic!("Invalid solution ID");
    };
    let input =
        fs::read_to_string(format!("inputs/day{day}.txt")).expect("Failed to open input file");
    let output = func(&input);
    println!("{output}");
}
