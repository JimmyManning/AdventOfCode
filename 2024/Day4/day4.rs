use std::fs::File;
use std::io::Read;
use std::collections::HashMap;
fn main() -> std::io::Result<()> {
    let mut file = File::open("input.txt")?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    let mut x = contents.split("\n");
    let mut total = 0;
    let mut grid : Vec<Vec<char>> = vec!();
    
    loop {
        let line = x.next();
        match line {
            Some(line) => {
                let mut current_line : Vec<char> = vec!();
                for i in line.chars() {
                    current_line.push(i);
                }
                grid.push(current_line);
            },
            None => {
                break;
            }
        }
    }
    let xmas = "XMAS";
    let left : Vec<(i32,i32)> = [(0, 0), (-1, 0), (-2, 0), (-3, 0)].to_vec();
    let right : Vec<(i32,i32)> = [(0, 0), (1, 0), (2, 0), (3, 0)].to_vec();
    let up : Vec<(i32,i32)> = [(0, 0), (0, -1), (0, -2), (0, -3)].to_vec();
    let down : Vec<(i32,i32)> = [(0, 0), (0, 1), (0, 2), (0, 3)].to_vec();

    let downleft : Vec<(i32,i32)> = [(0, 0), (-1, 1), (-2, 2), (-3, 3)].to_vec();
    let downright : Vec<(i32,i32)> = [(0, 0), (1, 1), (2, 2), (3, 3)].to_vec();
    let upleft : Vec<(i32,i32)> = [(0, 0), (-1, -1), (-2, -2), (-3, -3)].to_vec();
    let upright : Vec<(i32,i32)> = [(0, 0), (1, -1), (2, -2), (3, -3)].to_vec();

    let options : Vec<Vec<(i32,i32)>> = [left, right, up, down, upright, downright, upleft, downleft].to_vec();
    for i in 0..grid.len() {
        for ii in 0..grid[i].len() {
            print!("{} ", grid[i][ii]);
            
            for option in &options {
                let mut found = 1;
                let mut letter_index = 0;
                for step in option {
                    let xcord = ii as i32 + step.0;
                    let ycord = i as i32 + step.1;
                    if xcord < 0 || ycord < 0 || ycord >= grid.len() as i32 || xcord >= grid[i].len() as i32{
                        found = 0;
                        break;
                    }
                    if grid[ycord as usize][xcord as usize] != xmas.chars().nth(letter_index).expect("ERROR")
                    {
                        found = 0;
                        break;
                    }
                    letter_index += 1;
                }
                if found == 1 {
                    total += 1;
                }
            }
        }
        println!("");
    }
    let mut part_2_total = 0;
    for i in 1..grid.len() -1 {
        for ii in 1..grid[i].len() -1 {
            print!("{} ", grid[i][ii]);
            let mut minx = 0;
            let mut miny = 0;
            let mut maxx = 0;
            let mut maxy = 0;

            if grid[i][ii] == 'A' //&& i as i32 - 1 > 0 && ii as i32 - 1 > 0 && i + 1 < grid.len() - 1 && ii + 1 < grid[i].len() - 1 
            {
                if (grid[i-1][ii-1] == 'M' && grid[i+1][ii-1] == 'M' && grid[i-1][ii+1] == 'S' && grid[i+1][ii+1] == 'S' ||
                    grid[i-1][ii-1] == 'S' && grid[i+1][ii-1] == 'S' && grid[i-1][ii+1] == 'M' && grid[i+1][ii+1] == 'M' || 
                    grid[i-1][ii-1] == 'M' && grid[i+1][ii-1] == 'S' && grid[i-1][ii+1] == 'M' && grid[i+1][ii+1] == 'S' ||
                    grid[i-1][ii-1] == 'S' && grid[i+1][ii-1] == 'M' && grid[i-1][ii+1] == 'S' && grid[i+1][ii+1] == 'M'  ) {
                part_2_total += 1;
            }
            }

        }
        println!("");
    }

    println!("Total {}", total);
    println!("Part 2 Total {}", part_2_total);
    Ok(())
}