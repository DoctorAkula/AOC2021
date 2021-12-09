use crate::loadinput::loadinput;

pub fn day09p1() {
    let input = loadinput("./input/day9.txt");
    let mut heightmap : Vec<Vec<usize>> = Vec::new();
    let width = input[0].len();
    let height = input.len();
    println!("Width: {}, Height: {}", width, height);
    for line in &input {
        heightmap.push(line.chars().map(|val| (val as usize) - '0' as usize ).collect());
    }

    let mut low_levels : Vec<usize> = Vec::new();
    for (y, row) in heightmap.iter().enumerate() {
        for (x, val) in row.iter().enumerate() {
            //print!("{}",val);
            if (x+1) < width {
                if heightmap[y][x+1] <= *val {
                    continue;
                }
            }
            if (y+1) < height {
                if heightmap[y+1][x] <= *val {
                    continue;
                }
            }
            if x > 0 {
                if heightmap[y][x-1] <= *val {
                    continue;
                }
            }
            if y > 0 {
                if heightmap[y-1][x] <= *val {
                    continue;
                }
            }
            low_levels.push(val+1);
        }
        //println!("");
    }
    let danger_sum = low_levels.iter().fold(0, |acc, val| acc + val);
    println!("Danger Sum: {}", danger_sum);
}

#[derive(Clone, Copy)]
struct Coord {
    x : usize,
    y : usize,
}

fn flood_fill(visited : &mut Vec<Vec<bool>>, heightmap : &Vec<Vec<usize>>, pos_x : isize, pos_y : isize, width : isize, height : isize) -> usize {
    if ((pos_x < 0) || (pos_y < 0)) || ((pos_x >= width) || (pos_y >= height)) {
        return 0;
    }

    if  visited[pos_y as usize][pos_x as usize] || 
       (heightmap[pos_y as usize][pos_x as usize] == 9) {
        return 0;
    }

    visited[pos_y as usize][pos_x as usize] = true;
    let mut acc = flood_fill(visited, heightmap, pos_x + 1, pos_y, width, height);
    acc += flood_fill(visited, heightmap, pos_x - 1, pos_y, width, height);
    acc += flood_fill(visited, heightmap, pos_x, pos_y + 1, width, height);
    acc += flood_fill(visited, heightmap, pos_x, pos_y - 1, width, height);

    return acc + 1;
}

pub fn day09p2() {
    let input = loadinput("./input/day9.txt");
    let mut heightmap : Vec<Vec<usize>> = Vec::new();
    let width = input[0].len();
    let height = input.len();
    println!("Width: {}, Height: {}", width, height);
    for line in &input {
        heightmap.push(line.chars().map(|val| (val as usize) - '0' as usize ).collect());
    }

    let mut low_level_coords : Vec<Coord> = Vec::new();
    for (y, row) in heightmap.iter().enumerate() {
        for (x, val) in row.iter().enumerate() {
            if (x+1) < width {
                if heightmap[y][x+1] <= *val {
                    continue;
                }
            }
            if (y+1) < height {
                if heightmap[y+1][x] <= *val {
                    continue;
                }
            }
            if x > 0 {
                if heightmap[y][x-1] <= *val {
                    continue;
                }
            }
            if y > 0 {
                if heightmap[y-1][x] <= *val {
                    continue;
                }
            }
            low_level_coords.push(Coord{x,y});
        }
    }

    let mut basin_sizes : Vec<usize> = Vec::new();
    let mut visited : Vec<Vec<bool>> = vec![vec![false;width];height];
    for level_coord in low_level_coords {
        //println!("X: {}, Y: {}", level_coord.x, level_coord.y);
        basin_sizes.push(flood_fill(&mut visited, &heightmap, level_coord.x as isize, level_coord.y as isize, width as isize, height as isize));
    }
    basin_sizes.sort();
    basin_sizes.reverse();
    let score = basin_sizes[0] * basin_sizes[1] * basin_sizes[2];
    println!("Basin product: {}", score);
}
