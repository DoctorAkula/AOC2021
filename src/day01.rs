use crate::loadinput::loadinput;

pub fn day01p1() {
    let input = loadinput("./input/day1.txt");
    let vals : Vec<isize> = input.iter().map(|val| val.parse().unwrap()).collect();
    let mut prev = vals[0];
    let diffs : Vec<isize> = vals.iter().map(|val| {
        let ret = val - prev;
        prev = *val;
        ret
    }).collect();
    let mut count = 0;
    for diff in diffs {
        if diff > 0 {
            count += 1;
        }
    }
    println!("Count: {}", count);
}

pub fn day01p2() {
    let input = loadinput("./input/day1.txt");
    let vals : Vec<isize> = input.iter().map(|val| val.parse().unwrap()).collect();
    let mut sums : Vec<isize> = Vec::new();
    for (num, val) in vals.iter().enumerate() {
        let valp1 = vals.get(num + 1).unwrap();
        if let Some(valp2) = vals.get(num + 2){
            sums.push(val + valp1 + valp2);
        }else{
            break;
        }
    }
    let mut prev = sums[0];
    let diffs : Vec<isize> = sums.iter().map(|val| {
        let ret = val - prev;
        prev = *val;
        ret
    }).collect();
    let mut count = 0;
    for diff in diffs {
        if diff > 0 {
            count += 1;
        }
    }
    println!("Count: {}", count);
}
