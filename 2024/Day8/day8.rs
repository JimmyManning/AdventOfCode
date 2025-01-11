use std::fs::File;
use std::io::Read;
use std::collections::HashMap;

fn main() -> std::io::Result<()> {
    let mut file = File::open("input.txt")?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    let mut x = contents.split("\n");
    let mut total = 0;
    let mut row = 0;
    let mut antennas : HashMap<char, Vec<(usize, usize)>> = Default::default();
    let mut cols = 0;
    let mut grid : Vec<Vec<char>> = vec!();
    loop {
        let mut line = x.next();
        let mut currow : Vec<char> = vec!();
        match line {
            Some(line) => {
                cols = line.len();
                let mut col = 0;
                for i in line.chars() {
                    if i != '.' {
                        antennas.entry(i).or_insert_with(|| vec!()).push((col, row));
                    }
                    currow.push(i);
                    col += 1;
                }
            },
            None => {
                break;
            }
        }
        grid.push(currow);
        row += 1;
    }
    for i in 0..grid.len() {
        for ii in 0..grid[i].len()-1 {
            print!("{}", grid[i][ii]);
        }
        println!();
    }
    let mut gridback = grid.clone();
    for (antenna, locs) in antennas {
        //grid = gridback.clone();
        for (x, y) in &locs {
            for (x1, y1) in &locs {
                if x == x1 && y == y1 {
                    continue;
                }
                let mut times = 1;
                let mut xdiff : i32 = *x as i32 - *x1 as i32;
                let mut ydiff : i32 = *y as i32 - *y1 as i32;

                
                loop {
                    let mut newx : i32 = *x as i32 + (xdiff * times);
                    let mut newy : i32 = *y as i32 + (ydiff * times);
                    println!("{} {} {} {} {}", newx, newy, xdiff, ydiff, times);
                    if newx >= 0 && newx < grid[0].len() as i32 {
                        if newy >= 0 && newy < grid.len() as i32 {
                            if /*grid[newy as usize][newx as usize] != antenna &&*/ grid[newy as usize][newx as usize] != '#' {
                                grid[newy as usize][newx as usize] = '#';
                                total += 1;
                            }
                        }
                        else {
                            break;
                        }
                    }
                    else {
                        break;
                    }
                    times += 1;
                }
                times = 1;
                loop {
                    let mut newx : i32 = *x as i32 + (xdiff * times * -1);
                    let mut newy : i32 = *y as i32 + (ydiff * times * -1);
                    println!("{} {} {} {} {}", newx, newy, xdiff, ydiff, times);
                    if newx >= 0 && newx < grid[0].len() as i32 {
                        if newy >= 0 && newy < grid.len() as i32 {
                            if /*grid[newy as usize][newx as usize] != antenna &&*/ grid[newy as usize][newx as usize] != '#' {
                                grid[newy as usize][newx as usize] = '#';
                                total += 1;
                            }
                        }
                        else {
                            break;
                        }
                    }
                    else {
                        break;
                    }
                    times += 1;
                }
            }

        }
        println!("-----------");
        for i in 0..grid.len() {
            for ii in 0..grid[i].len() {
                print!("{}", grid[i][ii]);
            }
            println!();
        }
    }

    for i in 0..grid.len() {
        for ii in 0..grid[i].len() {
            print!("{}", grid[i][ii]);
        }
        println!();
    }
    

    println!("Total {}", total);
    Ok(())
}