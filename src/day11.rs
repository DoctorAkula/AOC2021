use crate::loadinput::loadinput;

pub fn day11p1() {
    let input = loadinput("./input/day11.txt");
    let height = input.len();
    let width = input[0].len();
    let mut dumbo_map : Vec<Vec<usize>> =
        input.iter().map(|line| line.chars().map(|num| (num as usize) - ('0' as usize)).collect()).collect();
    
    let mut flashes = 0;
    for _i in 0..100 {
//        println!("Step: {}", i);
//        for y in 0..height {
//            for x in 0..width {
//                print!("{},", dumbo_map[y][x]);
//            }
//            println!("\u{8} ");
//        }
//        println!("");

        for y in 0..height {
            for x in 0..width {
                dumbo_map[y][x] += 1;
            }
        }

        let mut done = false;
        let mut dumbo_map_clone = dumbo_map.clone();
        while !done {
            done = true;
            for y in 0..height {
                for x in 0..width {
                    if dumbo_map[y][x] > 9 {
                        done = false;
                        dumbo_map[y][x] = 0;
                        if let Some(row) = dumbo_map.get_mut(y+1) { if let Some(cell) = row.get_mut(x-1) { *cell += 1; dumbo_map_clone[y][x] = 0; } }
                        if let Some(row) = dumbo_map.get_mut(y+1) { if let Some(cell) = row.get_mut(x)   { *cell += 1; dumbo_map_clone[y][x] = 0;  } }
                        if let Some(row) = dumbo_map.get_mut(y+1) { if let Some(cell) = row.get_mut(x+1) { *cell += 1; dumbo_map_clone[y][x] = 0;  } }
                        if let Some(row) = dumbo_map.get_mut(y)   { if let Some(cell) = row.get_mut(x-1) { *cell += 1; dumbo_map_clone[y][x] = 0;  } }
                        if let Some(row) = dumbo_map.get_mut(y)   { if let Some(cell) = row.get_mut(x+1) { *cell += 1; dumbo_map_clone[y][x] = 0;  } }
                        if let Some(row) = dumbo_map.get_mut(y-1) { if let Some(cell) = row.get_mut(x-1) { *cell += 1; dumbo_map_clone[y][x] = 0;  } }
                        if let Some(row) = dumbo_map.get_mut(y-1) { if let Some(cell) = row.get_mut(x)   { *cell += 1; dumbo_map_clone[y][x] = 0;  } }
                        if let Some(row) = dumbo_map.get_mut(y-1) { if let Some(cell) = row.get_mut(x+1) { *cell += 1; dumbo_map_clone[y][x] = 0;  } }
                    }
                }
            }
        }
        for y in 0..height {
            for x in 0..width {
                if dumbo_map_clone[y][x] == 0 { dumbo_map[y][x] = 0; flashes += 1; }
            }
        }
        for y in 0..height {
            for x in 0..width {
                if dumbo_map[y][x] > 9 { dumbo_map[y][x] = 0; }
            }
        }
    }
    println!("Flashes: {}", flashes);
}

pub fn day11p2() {
    let input = loadinput("./input/day11.txt");
    let height = input.len();
    let width = input[0].len();
    let mut dumbo_map : Vec<Vec<usize>> =
        input.iter().map(|line| line.chars().map(|num| (num as usize) - ('0' as usize)).collect()).collect();
    
    let mut simul = 0;
    for i in 0..usize::MAX {
        simul = i + 1;
//        println!("Step: {}", i);
//        for y in 0..height {
//            for x in 0..width {
//                print!("{},", dumbo_map[y][x]);
//            }
//            println!("\u{8} ");
//        }
//        println!("");

        for y in 0..height {
            for x in 0..width {
                dumbo_map[y][x] += 1;
            }
        }

        let mut done = false;
        let mut dumbo_map_clone = dumbo_map.clone();
        while !done {
            done = true;
            for y in 0..height {
                for x in 0..width {
                    if dumbo_map[y][x] > 9 {
                        done = false;
                        dumbo_map[y][x] = 0;
                        if let Some(row) = dumbo_map.get_mut(y+1) { if let Some(cell) = row.get_mut(x-1) { *cell += 1; dumbo_map_clone[y][x] = 0; } }
                        if let Some(row) = dumbo_map.get_mut(y+1) { if let Some(cell) = row.get_mut(x)   { *cell += 1; dumbo_map_clone[y][x] = 0;  } }
                        if let Some(row) = dumbo_map.get_mut(y+1) { if let Some(cell) = row.get_mut(x+1) { *cell += 1; dumbo_map_clone[y][x] = 0;  } }
                        if let Some(row) = dumbo_map.get_mut(y)   { if let Some(cell) = row.get_mut(x-1) { *cell += 1; dumbo_map_clone[y][x] = 0;  } }
                        if let Some(row) = dumbo_map.get_mut(y)   { if let Some(cell) = row.get_mut(x+1) { *cell += 1; dumbo_map_clone[y][x] = 0;  } }
                        if let Some(row) = dumbo_map.get_mut(y-1) { if let Some(cell) = row.get_mut(x-1) { *cell += 1; dumbo_map_clone[y][x] = 0;  } }
                        if let Some(row) = dumbo_map.get_mut(y-1) { if let Some(cell) = row.get_mut(x)   { *cell += 1; dumbo_map_clone[y][x] = 0;  } }
                        if let Some(row) = dumbo_map.get_mut(y-1) { if let Some(cell) = row.get_mut(x+1) { *cell += 1; dumbo_map_clone[y][x] = 0;  } }
                    }
                }
            }
        }

        let mut flashes = 0;
        for y in 0..height {
            for x in 0..width {
                if dumbo_map_clone[y][x] == 0 { dumbo_map[y][x] = 0; flashes += 1; }
            }
        }
        if flashes == (width * height) { break; }

        for y in 0..height {
            for x in 0..width {
                if dumbo_map[y][x] > 9 { dumbo_map[y][x] = 0; }
            }
        }
    }
    println!("Iteration: {}", simul);
}
