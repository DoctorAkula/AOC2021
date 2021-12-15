use crate::loadinput::loadinput;
use std::collections::BinaryHeap;

pub fn day15p1() {
    let input = loadinput("./input/day15.txt");
    let width = input[0].len();
    let height = input.len();
    let end = (width-1,height-1);
    let edges : Vec<Vec<i64>> = input.iter().map(|line| line.chars().map(|num| num.to_string().parse().unwrap()).collect()).collect();
    let mut verts : Vec<Vec<i64>> = vec![vec![i64::MAX;width];height];
    let mut queue = BinaryHeap::new();
    queue.push((0,0,0));
    let mut total_cost = 0;
    while let Some((cost,x,y)) = queue.pop() {
        if end == (x,y) { total_cost = -cost; break; }
        if -cost > verts[y][x] { continue; }
        for (x1,y1) in [(x-1,y), (x+1,y), (x,y-1), (x,y+1)] {
            if let Some(row) = edges.get(y1) {
                if let Some(edge) = row.get(x1) {
                    let next_cost = -cost + edge;
                    if next_cost < verts[y1][x1] {
                        queue.push((-next_cost, x1, y1));
                        verts[y1][x1] = next_cost;
                    }
                }
            }
        }
    }
    println!("Total cost: {}", total_cost);
}

pub fn day15p2() {
    let input = loadinput("./input/day15.txt");
    let smol_width = input[0].len();
    let smol_height = input.len();
    let width = smol_width * 5;
    let height = smol_height * 5;
    let smol_edges : Vec<Vec<i64>> = input.iter().map(|line| line.chars().map(|num| num.to_string().parse().unwrap()).collect()).collect();
    let mut edges : Vec<Vec<i64>> = vec![vec![0;width];height];

    for megay in 0..5 {
        for megax in 0..5 {
            for smoly in 0..smol_height {
                for smolx in 0..smol_width {
                    let mut danger = (smol_edges[smoly][smolx] + (megax as i64) + (megay as i64)) % 9;
                    if danger == 0 { danger = 9; }
                    let x = smol_width * megax + smolx;
                    let y = smol_height * megay + smoly;
                    edges[y][x] = danger;
                }
            }
        }
    }

    let mut verts : Vec<Vec<i64>> = vec![vec![i64::MAX;width];height];
    let mut queue = BinaryHeap::new();
    let end = (width-1,height-1);
    queue.push((0,0,0));
    let mut total_cost = 0;
    while let Some((cost,x,y)) = queue.pop() {
        if end == (x,y) { total_cost = -cost; break; }
        if -cost > verts[y][x] { continue; }
        for (x1,y1) in [(x-1,y), (x+1,y), (x,y-1), (x,y+1)] {
            if let Some(row) = edges.get(y1) {
                if let Some(edge) = row.get(x1) {
                    let next_cost = -cost + edge;
                    if next_cost < verts[y1][x1] {
                        queue.push((-next_cost, x1, y1));
                        verts[y1][x1] = next_cost;
                    }
                }
            }
        }
    }
    println!("Total cost: {}", total_cost);
}
