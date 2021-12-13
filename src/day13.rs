use crate::loadinput::loadinput;

#[derive(Clone, Copy)]
struct Coord {
    x : usize,
    y : usize,
}

#[derive(Clone, Copy, Debug)]
enum FoldType {
    Up(usize),
    Left(usize),
}

fn print_map(paper_map : &Vec<Vec<bool>>, width : usize, height : usize){
    for y in 0..height {
        for x in 0..width {
            if paper_map[y][x] {
                print!("0");
            }else{
                print!(" ");
            }
        }
        println!("");
    }
}

fn fold_up(paper_map : &mut Vec<Vec<bool>>, width : usize, height : &mut usize, midpoint : usize){
    for y in 0..midpoint {
        for x in 0..width {
            paper_map[y][x] |= paper_map[*height-y-1][x];
            paper_map[*height-y-1][x] = false;
        }
    }
    *height /= 2;
}

fn fold_left(paper_map : &mut Vec<Vec<bool>>, width : &mut usize, height : usize, midpoint : usize){
    for y in 0..height {
        for x in 0..midpoint {
            paper_map[y][x] |= paper_map[y][*width-x-1];
            paper_map[y][*width-x-1] = false;
        }
    }
    *width /= 2;
}

fn count_map_points(paper_map : &Vec<Vec<bool>>, width : usize, height : usize) -> usize{
    let mut acc = 0;
    for y in 0..height {
        for x in 0..width {
            if paper_map[y][x] { acc += 1; }
        }
    }
    acc
}

pub fn day13p1() {
    let input = loadinput("./input/day13.txt");
    let mut width = 0;
    let mut height = 0;
    let mut coords : Vec<Coord> = Vec::new();
    let mut folds : Vec<FoldType> = Vec::new();
    {
        let mut cord_mode = true;
        for line in input.iter() {
            if line.len() == 0 { cord_mode = false; continue; }
            if cord_mode {
                let mut col = line.split(',');
                let x = col.next().unwrap().parse().unwrap();
                let y = col.next().unwrap().parse().unwrap();
                if x > width { width = x; }
                if y > height { height = y; }
                coords.push(Coord{x, y});
            }else{
                match &line[0..13] {
                    "fold along x=" => folds.push(FoldType::Left(line[13..].parse().unwrap())),
                    "fold along y=" => folds.push(FoldType::Up(line[13..].parse().unwrap())),
                    _ => eprintln!("uh oh"),
                }
            }
        }
        width += 1;
        height += 1;
    }
    //println!("Width: {}, Height: {}", width, height);
    let mut paper_map : Vec<Vec<bool>> = vec![vec![false;width];height];

    for coord in &coords {
        //println!("X: {}, Y: {}", coord.x, coord.y);
        paper_map[coord.y][coord.x] = true;
    }

    let fold = folds[0];
    //println!("Fold: {:#?}", fold);
    match fold {
       FoldType::Up(midpoint) => fold_up(&mut paper_map, width, &mut height, midpoint),
       FoldType::Left(midpoint) => fold_left(&mut paper_map, &mut width, height, midpoint),
    }
    println!("Visible dots: {}", count_map_points(&paper_map, width, height));
}

pub fn day13p2() {
    let input = loadinput("./input/day13.txt");
    let mut width = 0;
    let mut height = 0;
    let mut coords : Vec<Coord> = Vec::new();
    let mut folds : Vec<FoldType> = Vec::new();
    {
        let mut cord_mode = true;
        for line in input.iter() {
            if line.len() == 0 { cord_mode = false; continue; }
            if cord_mode {
                let mut col = line.split(',');
                let x = col.next().unwrap().parse().unwrap();
                let y = col.next().unwrap().parse().unwrap();
                if x > width { width = x; }
                if y > height { height = y; }
                coords.push(Coord{x, y});
            }else{
                match &line[0..13] {
                    "fold along x=" => folds.push(FoldType::Left(line[13..].parse().unwrap())),
                    "fold along y=" => folds.push(FoldType::Up(line[13..].parse().unwrap())),
                    _ => eprintln!("uh oh"),
                }
            }
        }
        width += 1;
        height += 1;
    }
    //println!("Width: {}, Height: {}", width, height);
    let mut paper_map : Vec<Vec<bool>> = vec![vec![false;width];height];

    for coord in &coords {
        //println!("X: {}, Y: {}", coord.x, coord.y);
        paper_map[coord.y][coord.x] = true;
    }

    for fold in &folds {
        //println!("Fold: {:#?}", fold);
        match fold {
            FoldType::Up(midpoint) => fold_up(&mut paper_map, width, &mut height, *midpoint),
            FoldType::Left(midpoint) => fold_left(&mut paper_map, &mut width, height, *midpoint),
        }
    }
    print_map(&paper_map, width, height);
}
