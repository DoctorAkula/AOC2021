use crate::loadinput::loadinput;
use std::collections::HashMap;

#[derive(Clone, Copy, Eq, Hash, PartialEq)]
struct Coord {
    x : isize,
    y : isize,
}

#[derive(Clone, Copy)]
struct LineSeg {
    ix : isize,
    iy : isize,
    fx : isize,
    fy : isize,
}

#[derive(Clone, Copy, Debug)]
enum LineType {
    Vertical,
    Horizontal,
    Diagonal,
}

impl LineSeg {
    fn get_line_type(&self) -> LineType {
        if self.ix == self.fx {
            return LineType::Vertical;
        }else if self.iy == self.fy {
            return LineType::Horizontal;
        }else{
            return LineType::Diagonal;
        }
    }
}

fn parse_lineseg(input : Vec<String>) -> Vec<LineSeg> {
    input.iter().map(|line| {
        let mut columns = line.split(" -> ");
        let initial_coords = columns.next().unwrap();
        let final_coords = columns.next().unwrap();
        let mut columns = initial_coords.split(',');
        let ix : isize = columns.next().unwrap().parse().unwrap();
        let iy : isize = columns.next().unwrap().parse().unwrap();
        let mut columns = final_coords.split(',');
        let fx : isize = columns.next().unwrap().parse().unwrap();
        let fy : isize = columns.next().unwrap().parse().unwrap();
        LineSeg{ix, iy, fx, fy}
    }).collect()
}

pub fn day05p1() {
    let input = loadinput("./input/day5.txt");
    let vent_lines : Vec<LineSeg> = parse_lineseg(input);
    let mut danger_zones : HashMap<Coord, usize> = HashMap::new();
    let mut sea_map : HashMap<Coord, usize> = HashMap::new();
    for vent_line in vent_lines {
        let line_type = vent_line.get_line_type();
        match line_type {
            LineType::Vertical => {
                let x = vent_line.ix;
                let dy = vent_line.fy - vent_line.iy;
                let ady = dy.abs();
                let dir = dy / ady;
                let mut y = vent_line.iy;
                for _i in 0..=ady {
                    let val = sea_map.get(&Coord{x,y}).or_else(|| Some(&0)).unwrap() + 1;
                    if val > 1 { danger_zones.insert(Coord{x,y}, val); }
                    sea_map.insert(Coord{x,y}, val);
                    y += dir;
                }
            },
            LineType::Horizontal => {
                let y = vent_line.iy;
                let mut x = vent_line.ix;
                let dx = vent_line.fx - vent_line.ix;
                let adx = dx.abs();
                let dir = dx / dx.abs();
                for _i in 0..=adx {
                    let val = sea_map.get(&Coord{x,y}).or_else(|| Some(&0)).unwrap() + 1;
                    if val > 1 { danger_zones.insert(Coord{x,y}, val); }
                    sea_map.insert(Coord{x,y}, val);
                    x += dir;
                }
            },
            LineType::Diagonal => (),
        }
    }
    println!("Map entries: ({})\nDanger zones: ({})", sea_map.len(), danger_zones.len());
}

pub fn day05p2() {
    let input = loadinput("./input/day5.txt");
    let vent_lines : Vec<LineSeg> = parse_lineseg(input);
    let mut danger_zones : HashMap<Coord, usize> = HashMap::new();
    let mut sea_map : HashMap<Coord, usize> = HashMap::new();
    for vent_line in vent_lines {
        let dx = vent_line.fx - vent_line.ix;
        let adx = dx.abs();
        let dy = vent_line.fy - vent_line.iy;
        let ady = dy.abs();

        let (dirx, diry, count) = if adx > ady {
            ((dx / adx) as f64, (dy / adx) as f64, adx)
        }else{
            ((dx / ady) as f64, (dy / ady) as f64, ady)
        };
        let mut x = vent_line.ix as f64;
        let mut y = vent_line.iy as f64;
        for _i in 0..=count {
            let val = sea_map.get(&Coord{x: x as isize,y: y as isize}).or_else(|| Some(&0)).unwrap() + 1;
            if val > 1 { danger_zones.insert(Coord{x: x as isize,y: y as isize}, val); }
            sea_map.insert(Coord{x: x as isize,y: y as isize}, val);
            x += dirx;
            y += diry;
        }
    }
    println!("Map entries: ({})\nDanger zones: ({})", sea_map.len(), danger_zones.len());
}
