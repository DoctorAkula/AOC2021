use crate::loadinput::loadinput;

pub fn day17p1() {
    let input = loadinput("./input/day17.txt");
    let (ylow, yhigh) : (isize, isize) = {
        let line = &input[0][13..];
        let mut comma_split = line.split(", ");
        comma_split.next();
        let y_range = &comma_split.next().unwrap()[2..];
        let mut doty_split = y_range.split("..");
        (doty_split.next().unwrap().parse().unwrap(),
         doty_split.next().unwrap().parse().unwrap())
    };

    let max_vel_y = {
        let mut max_vel_y = 0;
        for initvely in 0..1000 {
            let mut posy = (initvely * (initvely+1)) / 2;
            let mut vely = 0;
            loop{
                posy += vely;
                vely -= 1;
                if (posy <= yhigh) && (posy >= ylow) { 
                    max_vel_y = initvely;
                    break; 
                }
                else if posy < ylow { break; }
            }
        }
        max_vel_y
    };
    println!("Max y velocity: {}", max_vel_y);

    let highest_y = (max_vel_y * (max_vel_y+1)) / 2;
    println!("Highest Y: {}", highest_y);
}

pub fn day17p2() {
    let input = loadinput("./input/day17.txt");
    let (xlow, xhigh, ylow, yhigh) : (isize, isize, isize, isize) = {
        let line = &input[0][13..];
        let mut comma_split = line.split(", ");
        let x_range = &comma_split.next().unwrap()[2..];
        let y_range = &comma_split.next().unwrap()[2..];
        let mut dotx_split = x_range.split("..");
        let mut doty_split = y_range.split("..");
        (dotx_split.next().unwrap().parse().unwrap(),
         dotx_split.next().unwrap().parse().unwrap(),
         doty_split.next().unwrap().parse().unwrap(),
         doty_split.next().unwrap().parse().unwrap())
    };
    let mut accumulator = 0;
    for initvely in ylow..=1000 {
        for initvelx in 0..=xhigh {
            let mut posx = 0;
            let mut posy = 0;
            let mut velx = initvelx;
            let mut vely = initvely;
            loop{
                posy += vely;
                posx += velx;
                vely -= 1;
                if velx > 0 { velx -= 1; }
                if (posy <= yhigh) && (posy >= ylow) &&
                   (posx <= xhigh) && (posx >= xlow) { 
                    accumulator += 1;
                    break; 
                }
                else if (posy < ylow) || (posx > xhigh) { break; }
            }
        }
    }
    println!("Possible velocities: {}", accumulator);
}
