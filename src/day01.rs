use crate::loadinput::loadinput;

pub fn day01p1() {
    let input = loadinput("./input/day1-1.txt");
    for (lnum, line) in input.iter().enumerate() {
        println!("{}: {}", lnum, line);
    }
}
