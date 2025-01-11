use std::fs::File;
use std::io::Read;
use std::collections::HashMap;
fn traverse(mut grid : Vec<Vec<char>>, start : (usize, usize), end : (usize, usize), cur_dir : usize, lowests : &mut Vec<Vec<Vec<i32>>>) -> i32
{
    let mut value = -1;
    let mut dirs : Vec<(i32, i32)> = [(1, 0), (0, -1), (-1, 0),(0, 1),].to_vec();
    let mut dir_keys : Vec<char> = ['>', 'V', '^', '<'].to_vec();
    let mut lowest_path : i32 = -1;
    //println!("At {:?}", start);
    // if lowests[start.1][start.0][cur_dir] != 0
    // {
    //     println!("shorting at {:?}", start);
    //     return lowests[start.1][start.0][cur_dir];
    // }

    if grid[start.1][start.0] != '.' && grid[start.1][start.0] != 'S' && grid[start.1][start.0] != 'E' {
        return -1;
    }
    else {
        if grid[start.1][start.0] == 'E' {
            println!("Found the end!");
            return 0;
        }
        grid[start.1][start.0] = dir_keys[cur_dir];

        for i in 0..dirs.len() {
            let mut current_path : i32 = -1;
            let mut next_spot : (usize, usize) = (0, 0);

            if (start.0 as i32 + dirs[i].0) >= 0 && (start.0 as i32 + dirs[i].0) < grid[0].len() as i32 && 
                (start.1 as i32 + dirs[i].1) >= 0 && (start.1 as i32 + dirs[i].1) < grid.len() as i32 {
                next_spot.0 = (start.0 as i32 + dirs[i].0) as usize;
                next_spot.1 = (start.1 as i32 + dirs[i].1) as usize;
                if grid[next_spot.1][next_spot.0] != '.' && grid[next_spot.1][next_spot.0] != 'E' {
                    continue;
                }
                
                current_path = traverse(grid.clone(), next_spot, end, i, lowests);
                if current_path != -1 {
                    //println! ("Current path {} ", current_path);
                    if i == cur_dir {
                        current_path += 1;
                    }
                    else {
                        if dirs[i].0 == dirs[cur_dir].0 || dirs[i].1 == dirs[cur_dir].1 {
                            current_path += 2001;
                        } 
                        else 
                        {
                            current_path += 1001;
                        }
                    }

                    if lowest_path == -1 || current_path < lowest_path {
                        lowest_path = current_path;
                        println!("new lowest {}", lowest_path);
                    }
                }
            }
        }
        lowests[start.1 as usize][start.0 as usize][cur_dir] = lowest_path;
    }
    return lowest_path;
}


fn main() -> std::io::Result<()> {
    let mut file = File::open("input.txt")?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    let mut x = contents.split("\n");
    let mut total = 0;
    let mut grid : Vec<Vec<char>> = vec!();
    let mut lowest : Vec<Vec<Vec<i32>>> = vec!();
    let mut rows = 0;
    let mut cols = 0;
    let mut start : (usize, usize) = (0, 0); 
    let mut end : (usize, usize) = (0, 0);
    loop {
        let mut line = x.next();
        match line {
            Some(line) => {
                cols = 0;
                let mut row : Vec<char> = vec!();
                let mut lowestrow : Vec<Vec<i32>> = vec!();
                for col in line.chars() {
                    if col == 'S' {
                        start = (cols, rows);
                    } else if col == 'E' {
                        end = (cols, rows);
                    }
                    lowestrow.push([0, 0, 0, 0].to_vec());
                    row.push(col);
                    cols += 1;
                }
                grid.push(row);
                lowest.push(lowestrow);
                rows += 1;
            },
            None => {
                break;
            }
        }

    }

    total = traverse(grid.clone(), start, end, 0, &mut lowest);

    for row in grid {
        for col in row {
            print!("{}", col);
        }
        println!();
    }

    println!("Total {} {:?} {:?} 147468 133480", total, start, end);
    Ok(())
}