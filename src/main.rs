use std::env;
use std::time::Instant;

mod common;
mod one;
mod two;

fn one() -> String {
    //let masses = common::read_list_of_numbers("data/one.input", "\n");
    //format!("part1: {}", one::one(&masses),)

    let numbers = common::read_list_of_numbers("data/one.input", "\n");
    return format!("{}", one::one(&numbers));
}

fn two() -> String {
    let commands = common::read_list_of_numbers("data/two.input", ",");

    return format!("{}", two::two(&mut Vec::from(&commands[..])));
}

fn do_day(days: &[fn() -> String], day: usize) {
    let now = Instant::now();
    println!(
        "Result of day {}: {} (time: {} μs)",
        day,
        days[day - 1](),
        now.elapsed().as_micros()
    );
}

fn main() {
    println!("https://adventofcode.com/2019");

    let days: Vec<fn() -> String> = vec![one, two];

    let args: Vec<String> = env::args().skip(1).collect();

    // No argument -> execute all day problems.
    if args.is_empty() {
        for i in 1..=days.len() {
            do_day(&days, i)
        }
    } else {
        for arg in args {
            let day = arg.parse::<usize>().unwrap();
            if day > days.len() {
                panic!("Unknown day: {}", day)
            }
            do_day(&days, day)
        }
    }
}
