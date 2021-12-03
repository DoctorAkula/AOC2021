mod loadinput;
mod day01;

use crate::day01::*;

use std::env;
use std::process::exit;

fn main() {
    let funcs : [Option<fn()>; 50] = [
        Some(day01p1), None,
        None         , None,
        None         , None,
        None         , None,
        None         , None,
        None         , None,
        None         , None,
        None         , None,
        None         , None,
        None         , None,
        None         , None,
        None         , None,
        None         , None,
        None         , None,
        None         , None,
        None         , None,
        None         , None,
        None         , None,
        None         , None,
        None         , None,
        None         , None,
        None         , None,
        None         , None,
        None         , None,
        None         , None,
    ];

    let args : Vec<String> = env::args().collect();
    if args.len() != 3 {
        eprintln!("{} Usage: <Day> <Problem>", args[0]);
        exit(1);
    }

    let day : usize = args[1].parse().unwrap();
    if (1 > day) || (day > 25) {
        eprintln!("The Day can only be between 1-25 (you entered: {})", day);
        exit(2);
    }

    let problem : usize = args[2].parse().unwrap();
    if (1 > problem) || (problem > 2) {
        eprintln!("The problem can only be between 1-2 (you entered: {})", problem);
        exit(3);
    }

    let index = (day - 1) * 2 + (problem - 1);
    match funcs[index] {
        Some(func) => func(),
        None => {
            eprintln!("Day {}, Problem {} not implemented yet!", day, problem);
            exit(4);
        }
    }
}
