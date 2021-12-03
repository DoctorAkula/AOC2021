use crate::loadinput::loadinput;

pub fn day03p1() {
    let input = loadinput("./input/day3.txt");
    let bitlen = input[0].len();
    let mut gamma = 0;
    for ind in 0..bitlen {
        let mut zerocount = 0;
        let mut onecount = 0;
        for line in &input {
            let line = line.as_bytes();
            if line[ind] == '0' as u8 {
                zerocount += 1;
            }else if line[ind] == '1' as u8 {
                onecount += 1;
            }
        }
        if onecount > zerocount {
            gamma |= 1 << (bitlen - ind - 1);
        }
    }
    let epsilon = (!gamma) & ((1 << bitlen) - 1);
    println!("Gamma: {}", gamma);
    println!("Epsilon: {}", epsilon);
    println!("Product: {}", gamma * epsilon);
}

pub fn day03p2() {
    let input = loadinput("./input/day3.txt");
    let bitlen = input[0].len();
    let mut input_clone = input.clone();
    for ind in 0..bitlen {
        let mut zerocount = 0;
        let mut onecount = 0;
        for line in &input_clone {
            let line = line.as_bytes();
            if line[ind] == '0' as u8 {
                zerocount += 1;
            }else if line[ind] == '1' as u8 {
                onecount += 1;
            }
        }

        let comp = if onecount >= zerocount {
            '1' as u8
        }else{
            '0' as u8
        };

        input_clone = input_clone.into_iter().filter(|line| {
            let line = line.as_bytes();
            line[ind] == comp
        }).collect();

        if input_clone.len() == 1 {
            break;
        }
    }
    let oxygen = usize::from_str_radix(&input_clone[0], 2).unwrap();

    let mut input_clone = input.clone();
    for ind in 0..bitlen {
        let mut zerocount = 0;
        let mut onecount = 0;
        for line in &input_clone {
            let line = line.as_bytes();
            if line[ind] == '0' as u8 {
                zerocount += 1;
            }else if line[ind] == '1' as u8 {
                onecount += 1;
            }
        }

        let comp = if onecount < zerocount {
            '1' as u8
        }else{
            '0' as u8
        };

        input_clone = input_clone.into_iter().filter(|line| {
            let line = line.as_bytes();
            line[ind] == comp
        }).collect();

        if input_clone.len() == 1 {
            break;
        }
    }
    let c02scrub = usize::from_str_radix(&input_clone[0], 2).unwrap();

    println!("Oxygen: {}", oxygen);
    println!("c02scrub: {}", c02scrub);
    println!("Product: {}", oxygen * c02scrub);
}
