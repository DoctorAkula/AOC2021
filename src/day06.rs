use crate::loadinput::loadinput;

pub fn day06p1() {
    let input = loadinput("./input/day6.txt");
    let mut ages : Vec<u8> = input[0].split(',').map(|val| val.parse().unwrap()).collect();
    for _i in 0..80 {
        let mut fishadd = 0;
        for age in ages.iter_mut() {
            if *age == 0 {
                fishadd += 1;
                *age = 7;
            }
            *age -= 1;
        }
        for _j in 0..fishadd {
            ages.push(8);
        }
    }
    println!("Fish population: {}", ages.len());
}

//6+ hours solution
//pub fn day06p2() {
//    let input = loadinput("./input/day6.txt");
//    let ages : Vec<u8> = input[0].split(',').map(|val| val.parse().unwrap()).collect();
//    let mut fish_pop = 0;
//    for (num, age) in ages.into_iter().enumerate() {
//        let mut children : Vec<u8> = vec![age];
//        for i in 0..256 {
//            let mut childadd = 0;
//            for child_age in children.iter_mut() {
//                if *child_age == 0 {
//                    childadd += 1;
//                    *child_age = 7;
//                }
//                *child_age -= 1;
//            }
//            for _j in 0..childadd {
//                children.push(8);
//            }
//        }
//        fish_pop += children.len();
//    }
//    println!("Fish population: {}", fish_pop);
//}

//Speed demon
pub fn day06p2() {
    let input = loadinput("./input/day6.txt");
    let fish_ages : Vec<usize> = input[0].split(',').map(|val| val.parse().unwrap()).collect();
    let mut fish_counters : [usize; 9] = [0;9];
    for fish_age in fish_ages {
        fish_counters[fish_age] += 1;
    }
    for _i in 0..256 {
        let cur_fish_count = fish_counters[0];
        fish_counters[0] = fish_counters[1];
        fish_counters[1] = fish_counters[2];
        fish_counters[2] = fish_counters[3];
        fish_counters[3] = fish_counters[4];
        fish_counters[4] = fish_counters[5];
        fish_counters[5] = fish_counters[6];
        fish_counters[6] = fish_counters[7];
        fish_counters[7] = fish_counters[8];
        fish_counters[8] = cur_fish_count;
        fish_counters[6] += cur_fish_count;
    }
    let fish_population : usize = fish_counters.iter().fold(0, |acc,val| acc + val);
    println!("Fish population: {}", fish_population);
}
