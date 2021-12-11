mod loadinput;
mod day01; mod day02; mod day03; mod day04; mod day05;
mod day06; mod day07; mod day08; mod day09; mod day10;
mod day11;

use crate::day01::*; use crate::day02::*;
use crate::day03::*; use crate::day04::*;
use crate::day05::*; use crate::day06::*;
use crate::day07::*; use crate::day08::*;
use crate::day09::*; use crate::day10::*;
use crate::day11::*;

use std::env;
use std::process::exit;
use std::time::Instant;

fn main() {
    let funcs : [Option<fn()>; 50] = [
        Some(day01p1), Some(day01p2),
        Some(day02p1), Some(day02p2),
        Some(day03p1), Some(day03p2),
        Some(day04p1), Some(day04p2),
        Some(day05p1), Some(day05p2),
        Some(day06p1), Some(day06p2),
        Some(day07p1), Some(day07p2),
        Some(day08p1), Some(day08p2),
        Some(day09p1), Some(day09p2),
        Some(day10p1), Some(day10p2),
        Some(day11p1), Some(day11p2),
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
    if args.len() == 1{
        println!("Running all available solutions...");
        let timeit = Instant::now();
        for (fnum, func) in funcs.into_iter().enumerate() {
            if let Some(func) = func{
                println!("\nDay: {}, Problem: {}", (fnum >> 1) + 1, (fnum & 1) + 1);
                func();
            }
        }
        println!("\n Timing results for all:\n===============================\n Duration: {:.15}s", timeit.elapsed().as_secs_f64());
        exit(0)
    }else if args.len() != 3 {
        eprintln!("{} Usage: [<Day> <Problem>]", args[0]);
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
        Some(func) => {
            let timeit = Instant::now();
            func();
            println!("\n Timing results:\n===============================\n Duration: {:.15}s", timeit.elapsed().as_secs_f64());
        }
        None => {
            eprintln!("Day {}, Problem {} not implemented yet!", day, problem);
            exit(4);
        }
    }
}
