use crate::loadinput::loadinput;

pub fn day07p1() {
    let input = loadinput("./input/day7.txt");
    let mut sub_positions : Vec<isize> = input[0].split(',').map(|val| val.parse().unwrap()).collect();
    sub_positions.sort();
    let lowest = sub_positions[0];
    let highest = *sub_positions.last().unwrap();
    let mut differences : Vec<isize> = Vec::new();
    for pos in lowest..=highest {
        differences.push(sub_positions.iter().fold(0, |acc, val| acc + (val - pos).abs()));
    }
    differences.sort();
    println!("Lowest fuel cost: {}", differences[0]);
}

pub fn day07p2() {
    let input = loadinput("./input/day7.txt");
    let mut sub_positions : Vec<isize> = input[0].split(',').map(|val| val.parse().unwrap()).collect();
    sub_positions.sort();
    let lowest = sub_positions[0];
    let highest = *sub_positions.last().unwrap();
    let mut fuel_costs : Vec<isize> = Vec::new();
    for pos in lowest..=highest {
        fuel_costs.push(sub_positions.iter().fold(0, |acc, val| {
            let diff = (val - pos).abs();
            acc + ((diff + 1) * diff) / 2   //Fuel consumption of the submarines are triangle numbers
        }));
    }
    fuel_costs.sort();
    println!("Lowest fuel cost: {}", fuel_costs[0]);
}
