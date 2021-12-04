use crate::loadinput::loadinput;
use std::collections::HashMap;

#[derive(Clone, Copy)]
struct Coord {
    x : usize,
    y : usize,
}

#[derive(Clone, Copy)]
struct Entry {
    key : usize,
    coord : Coord,
}

pub fn day04p1() {
    let input = loadinput("./input/day4.txt");
    let mut input_iter = input.iter();
    let bingo_calls : Vec<usize> = input_iter.next().unwrap().split(',').map(|val| val.parse().unwrap()).collect();
    let mut bingo_tables : Vec<HashMap<usize, Coord>> = Vec::new();
    let mut row = 1;
    {
        let mut bingo_table : &mut HashMap<usize, Coord> = &mut HashMap::new();
        for line in input_iter {
            if line == "" {
                bingo_tables.push(HashMap::new());
                bingo_table = bingo_tables.last_mut().unwrap();
                row = 1;
                continue;
            }
            for (col, val) in line.split_whitespace().enumerate() {
                bingo_table.insert(val.parse().unwrap(), Coord{x: col+1, y: row});
            }
            row += 1;
        }
    }

    let mut bingo_marked_tables : Vec<Vec<Entry>> = vec![Vec::new();bingo_tables.len()];
    let mut winning_table : isize = -1;
    let mut final_call = 0;
    for bingo_call in bingo_calls {
        for (bingo_table, bingo_marked_table) in bingo_tables.iter().zip(bingo_marked_tables.iter_mut()) {
            if let Some(entry) = bingo_table.get(&bingo_call) {
                bingo_marked_table.push(Entry{key : bingo_call, coord : *entry});
            }
        }
        for (num, bingo_marked_table) in bingo_marked_tables.iter().enumerate() {
            let mut y_matches = [0;5];
            let mut x_matches = [0;5];
            for bingo_mark in bingo_marked_table {
                if bingo_mark.coord.x == 1 { x_matches[0] += 1 }
                if bingo_mark.coord.x == 2 { x_matches[1] += 1 }
                if bingo_mark.coord.x == 3 { x_matches[2] += 1 }
                if bingo_mark.coord.x == 4 { x_matches[3] += 1 }
                if bingo_mark.coord.x == 5 { x_matches[4] += 1 }

                if bingo_mark.coord.y == 1 { y_matches[0] += 1 }
                if bingo_mark.coord.y == 2 { y_matches[1] += 1 }
                if bingo_mark.coord.y == 3 { y_matches[2] += 1 }
                if bingo_mark.coord.y == 4 { y_matches[3] += 1 }
                if bingo_mark.coord.y == 5 { y_matches[4] += 1 }
            }
            if x_matches[0] == 5 { winning_table = num as isize; break; }
            if x_matches[1] == 5 { winning_table = num as isize; break; }
            if x_matches[2] == 5 { winning_table = num as isize; break; }
            if x_matches[3] == 5 { winning_table = num as isize; break; }
            if x_matches[4] == 5 { winning_table = num as isize; break; }

            if y_matches[0] == 5 { winning_table = num as isize; break; }
            if y_matches[1] == 5 { winning_table = num as isize; break; }
            if y_matches[2] == 5 { winning_table = num as isize; break; }
            if y_matches[3] == 5 { winning_table = num as isize; break; }
            if y_matches[4] == 5 { winning_table = num as isize; break; }
        }
        if winning_table >= 0 { 
            final_call = bingo_call;
            break; 
        }
    }
    let mut winning_table_clone = bingo_tables[winning_table as usize].clone();
    println!("Winning table: {}, with call: {}", winning_table + 1, final_call);
    for bingo_mark in bingo_marked_tables[winning_table as usize].iter() {
        print!("({},{}) = {}, ", bingo_mark.coord.x, bingo_mark.coord.y, bingo_mark.key);
        winning_table_clone.remove(&bingo_mark.key);
    }
    println!("");
    let table_sum : usize = winning_table_clone.iter().fold(0, |acc, (key, _val) | acc + key);
    println!("Final Score: {}", table_sum * final_call);
}

pub fn day04p2() {
    let input = loadinput("./input/day4.txt");
    let mut input_iter = input.iter();
    let bingo_calls : Vec<usize> = input_iter.next().unwrap().split(',').map(|val| val.parse().unwrap()).collect();
    let mut bingo_tables : Vec<HashMap<usize, Coord>> = Vec::new();
    let mut row = 1;
    {
        let mut bingo_table : &mut HashMap<usize, Coord> = &mut HashMap::new();
        for line in input_iter {
            if line == "" {
                bingo_tables.push(HashMap::new());
                bingo_table = bingo_tables.last_mut().unwrap();
                row = 1;
                continue;
            }
            for (col, val) in line.split_whitespace().enumerate() {
                bingo_table.insert(val.parse().unwrap(), Coord{x: col+1, y: row});
            }
            row += 1;
        }
    }

    let mut bingo_marked_tables : Vec<Vec<Entry>> = vec![Vec::new();bingo_tables.len()];
    let mut winning_table : isize = -1;
    let mut final_call = 0;
    for bingo_call in &bingo_calls {
        for (bingo_table, bingo_marked_table) in bingo_tables.iter().zip(bingo_marked_tables.iter_mut()) {
            if let Some(entry) = bingo_table.get(&bingo_call) {
                bingo_marked_table.push(Entry{key : *bingo_call, coord : *entry});
            }
        }

        let mut remove_tables : Vec<usize> = Vec::new();
        for (num, bingo_marked_table) in bingo_marked_tables.iter().enumerate() {
            let mut y_matches = [0;5];
            let mut x_matches = [0;5];
            for bingo_mark in bingo_marked_table {
                if bingo_mark.coord.x == 1 { x_matches[0] += 1 }
                if bingo_mark.coord.x == 2 { x_matches[1] += 1 }
                if bingo_mark.coord.x == 3 { x_matches[2] += 1 }
                if bingo_mark.coord.x == 4 { x_matches[3] += 1 }
                if bingo_mark.coord.x == 5 { x_matches[4] += 1 }

                if bingo_mark.coord.y == 1 { y_matches[0] += 1 }
                if bingo_mark.coord.y == 2 { y_matches[1] += 1 }
                if bingo_mark.coord.y == 3 { y_matches[2] += 1 }
                if bingo_mark.coord.y == 4 { y_matches[3] += 1 }
                if bingo_mark.coord.y == 5 { y_matches[4] += 1 }
            }
            if x_matches[0] == 5 {remove_tables.push(num); continue;}
            if x_matches[1] == 5 {remove_tables.push(num); continue;}
            if x_matches[2] == 5 {remove_tables.push(num); continue;}
            if x_matches[3] == 5 {remove_tables.push(num); continue;}
            if x_matches[4] == 5 {remove_tables.push(num); continue;}

            if y_matches[0] == 5 {remove_tables.push(num); continue;}
            if y_matches[1] == 5 {remove_tables.push(num); continue;}
            if y_matches[2] == 5 {remove_tables.push(num); continue;}
            if y_matches[3] == 5 {remove_tables.push(num); continue;}
            if y_matches[4] == 5 {remove_tables.push(num); continue;}
        }

        remove_tables.sort(); remove_tables.reverse();
        for index in remove_tables {
            if bingo_marked_tables.len() > 1{
                bingo_marked_tables.swap_remove(index);
                bingo_tables.swap_remove(index);
            }else{
                winning_table = 0;
                break;
            }
        }

        if  winning_table >= 0  { 
                final_call = *bingo_call;
                break; 
        }
    }
    let mut winning_table_clone = bingo_tables[winning_table as usize].clone();
    println!("Losing table: {}, with call: {}", winning_table + 1, final_call);
    for bingo_mark in bingo_marked_tables[winning_table as usize].iter() {
        print!("({},{}) = {}, ", bingo_mark.coord.x, bingo_mark.coord.y, bingo_mark.key);
        winning_table_clone.remove(&bingo_mark.key);
    }
    println!("");
    let table_sum : usize = winning_table_clone.iter().fold(0, |acc, (key, _val) | acc + key);
    println!("Final Score: {}", table_sum * final_call);
}
