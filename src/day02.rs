use crate::loadinput::loadinput;

pub fn day02p1() {
    let input = loadinput("./input/day2.txt");
    let mut x : usize = 0;
    let mut y : usize = 0;
    for line in input {
        let column : Vec<&str> = line.split_whitespace().collect();
        match column[0] {
            "forward" => x += column[1].parse::<usize>().unwrap(),
            "down" => y += column[1].parse::<usize>().unwrap(),
            "up" => y -= column[1].parse::<usize>().unwrap(),
            _ => println!("Bad input!"),
        }
    }
    println!("Product: {}", x * y);
}

pub fn day02p2() {
    let input = loadinput("./input/day2.txt");
    let mut aim : usize = 0;
    let mut x : usize = 0;
    let mut y : usize = 0;
    for line in input {
        let column : Vec<&str> = line.split_whitespace().collect();
        match column[0] {
            "forward" => { 
                let val = column[1].parse::<usize>().unwrap();
                x += val;
                y += aim * val;
            },
            "down" => aim += column[1].parse::<usize>().unwrap(),
            "up" => aim -= column[1].parse::<usize>().unwrap(),
            _ => println!("Bad input!"),
        }
    }
    println!("Product: {}", x * y);
}
