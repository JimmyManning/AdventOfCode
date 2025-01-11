use std::fs::File;
use std::io::Read;
use std::collections::HashMap;

fn Traverse( grid : &mut Vec<Vec<(char, bool)>>,  start : (i32, i32), prev : (i32, i32)) -> i32 {
    //println!(" At {} {}", start.0, start.1);
    grid[start.1 as usize][start.0 as usize].1 = true;
    if grid[start.1 as usize][start.0 as usize].0 == '9' {
        println!("9 at {:?}", start);
        return 1;
    }
    let mut val = 0;
    let mut ways : Vec<(i32, i32)> = [(1, 0), (0, 1), (-1, 0), (0, -1)].to_vec();

    for way in ways {
        //println!("way {:?}", way);
        let newx : i32 = start.0 + way.0;
        let newy : i32 = start.1 + way.1;
        if newx >= 0 && newy >= 0 && newx < grid[0].len() as i32 && newy < grid.len() as i32 {
            let x1 : usize = start.0 as usize;
            let y1 : usize = start.1 as usize;
            let x2 : usize = newx as usize;
            let y2 : usize = newy as usize;
            //println!("checking {} {}", x2, y2);
            if grid[y2][x2].0 != '.' {
                //println!("not . {} {}", (grid[y1][x1].0 as i32 - grid[y2][x2].0 as i32).abs(), grid[x2][y2].1);
                if grid[y2][x2].0 as i32 - grid[y1][x1].0 as i32 == 1 && /*grid[y2][x2].1 == false*/ (x2 as i32 != prev.0 || y2 as i32 != prev.1) {
                    val += Traverse(grid, (x2 as i32, y2 as i32), (x1 as i32, y1 as i32));
                }
            }
        }

    }
    return val;
}


fn main() -> std::io::Result<()> {
    let mut file = File::open("input.txt")?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    let mut x = contents.split("\n");
    let mut total = 0;
    let mut grid : Vec<Vec<(char, bool)>> = vec!();
    let mut starts : Vec<(i32, i32)> = vec!();
    let mut rows : i32 = 0;
    let mut cols : i32 = 0;
    loop {
        let mut line = x.next();
        match line {
            Some(line) => {
                let mut row : Vec<(char, bool)> = vec!();
                cols = 0;
                for col in line.chars() {
                    if col == '0' {
                        starts.push((cols, rows));
                    }
                    row.push((col, false));
                    cols += 1;
                }
                grid.push(row);
            },
            None => {
                break;
            }
        }
        rows += 1;
    }
    
    for start in starts {
        let mut gridback = grid.clone();
        let mut trailheadscore = 0;
        trailheadscore = Traverse(&mut gridback, start, start);
        println!("Trail head score {} {:?}", trailheadscore, start);
        total += trailheadscore;
    }

    for y in 0..grid.len() {
        for x in 0..grid[0].len() {
            print!("{}", grid[y][x].1 as u8);
        }
        println!();
    }
    
    println!("Total {}", total);
    Ok(())
}