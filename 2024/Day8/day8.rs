use std::fs::File;
use std::io::Read;
use std::collections::HashMap;

fn main() -> std::io::Result<()> {
    let mut file = File::open("puzzle.txt")?;
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
    for (antenna, locs) in antennas {
        for (x, y) in &locs {
            for (x1, y1) in &locs {
                if *x != *x1 && *y != *y1 {
                    let mut d : f64 = (f64::powf((*x as i32 - *x1 as i32) as f64, 2.0) + f64::powf((*y as i32 - *y1 as i32) as f64, 2.0)).sqrt();
                    let mut distance = d as i32;
                    let slope = (*y as i32 - *y1 as i32) / (*x as i32 - *x1 as i32);
                    let b : i32 = *y as i32 - slope * *x as i32;
                    //let antinode1 = slope * x + b;

                    let mut testdistance = 0;
                    let mut basex = *x;
                    let mut basey = *y;
                    let mut testx : i32 = 0 as i32;
                    let mut testy : i32 = 0;
                    for i in 0..cols {
                        testy = slope * testx + b;
                        let mut testd : f64 = (f64::powf((basex as i32 - testx as i32) as f64, 2.0) + f64::powf((basey as i32 - testy as i32) as f64, 2.0)).sqrt();
                        testdistance = testd as i32;
                        if testdistance == distance {
                            break;
                        }
                        testx += 1;
                    }
                    
                    //let testy = (1-distance as i32) * (*y) as i32 + distance*(*x1 as i32);
                    if testdistance == distance {
                    if testx >= 0 && testx < cols as i32 && testy >= 0 && testy < row as i32 {
                        total += 1;
                        println!("yes");
                        grid[testy as usize][testx as usize] = '#';
                    }
                    println!("{} {} {} {} {} {} {} () {} {}", antenna, x, y, x1, y1, distance, slope, testx, testy);
                }
                }
                
            }
        }
    }

    for i in 0..grid.len() {
        for ii in 0..grid[i].len()-1 {
            print!("{}", grid[i][ii]);
        }
        println!();
    }
    

    println!("Total {}", total);
    Ok(())
}