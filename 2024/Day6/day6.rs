use std::fs::File;
use std::io::Read;
use std::collections::HashMap;
fn will_block_cause_loop(inmap : Vec<Vec<char>>, startx : i32, starty : i32, blockx : i32, blocky :i32, mutmap : &Vec<Vec<char>>) -> bool {
    let mut gaurd_x : i32 = startx;
    let mut gaurd_y : i32 = starty;
    let mut dir = 0;
    let mut dirs : Vec<(i32, i32)> = [(-1, 0), (0, 1), (1, 0), (0, -1)].to_vec();
    let mut map = inmap.clone();
    let mut hitblockdirs : Vec<Vec<Vec<bool>>> = vec!();
    let mut row : Vec<Vec<bool>> = vec!();
    let mut hitdirs : Vec<bool> = [false, false, false, false].to_vec();
    for i in 0..map[0].len() {
        row.push(hitdirs.clone());
    }
    for i in 0..map.len() {
        hitblockdirs.push(row.clone());
    }

    let mut hitdirs : Vec<bool> = [false, false, false, false].to_vec();
    let mut return_value = false;
    if !(blockx < 0 || blocky < 0 || blockx as usize >= map[0].len() || blocky as usize >= map.len()) && map[blocky as usize][blockx as usize] != '#' && mutmap[blocky as usize][blockx as usize] != 'O' {
        map[blocky as usize][blockx as usize] = '#';
        loop
        {
            if gaurd_x < 0 || gaurd_y < 0 || gaurd_x as usize >= map[0].len() || gaurd_y as usize >= map.len() {
                break;
            }

            if map[gaurd_y as usize][gaurd_x as usize] == '#' {
                    if (hitblockdirs[gaurd_y as usize][gaurd_x as usize][dir %4]){
                        return_value = true;
                        break;
                    }
                    hitblockdirs[gaurd_y as usize][gaurd_x as usize][dir %4] = true;

                gaurd_y -= dirs[dir % 4].0;
                gaurd_x -= dirs[dir % 4].1;
                dir += 1;
            }
            else {
                    map[gaurd_y as usize][gaurd_x as usize] = 'X';                
            }
            gaurd_x += dirs[dir % 4].1;
            gaurd_y += dirs[dir % 4].0;
        }
    }
    return return_value;
}

fn main() -> std::io::Result<()> {
    let mut file = File::open("input.txt")?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    let mut x = contents.split("\n");
    let mut total = 0;
    let mut map : Vec<Vec<char>> = vec!();
    let mut gaurd_x : i32 = 0;
    let mut gaurd_y : i32 = 0;
    let mut gaurd_start_x : i32 = 0;
    let mut gaurd_start_y : i32 = 0;
    let mut y = 0;

    loop {
        let mut line = x.next();
        match line
        {
            Some(line) => {
                let mut row : Vec<char> = vec!();
                let mut x = 0;
                for i in line.chars() {
                    if i == '^' {
                        gaurd_x = x;
                        gaurd_y = y;
                        gaurd_start_x = x;
                        gaurd_start_y = y;
                    }
                    row.push(i);
                    x += 1;
                }
                map.push(row);
                println!("{}", line);
            },
            None => {
                break;
            }

        }
        y += 1;
    }
    println!(" Gaurd starting at {} {}", gaurd_x, gaurd_y);
    let mut map1 = map.clone();
    let mut map2 = map.clone();
    let mut dir = 0;
    let mut options = 0;
    let mut dirs : Vec<(i32, i32)> = [(-1, 0), (0, 1), (1, 0), (0, -1)].to_vec();
    
    loop
    {
        if gaurd_x < 0 || gaurd_y < 0 || gaurd_x as usize >= map[0].len() || gaurd_y as usize >= map.len() {
            break;
        }

        if map[gaurd_y as usize][gaurd_x as usize] == '#' {
            gaurd_y -= dirs[dir % 4].0;
            gaurd_x -= dirs[dir % 4].1;
            dir += 1;
        }
        else {
            map[gaurd_y as usize][gaurd_x as usize] = 'X';            
        }
        gaurd_x += dirs[dir % 4].1;
        gaurd_y += dirs[dir % 4].0;

        if will_block_cause_loop(map2.clone(), gaurd_start_x, gaurd_start_y, gaurd_x, gaurd_y, &map1)
        {
            if map1[(gaurd_y) as usize][(gaurd_x) as usize] != 'O' {
            options += 1;
            }
            map1[(gaurd_y) as usize][(gaurd_x) as usize] = 'O';
        }
        println!(" Gaurd  at {} {}", gaurd_x, gaurd_y);

    }

    for row in map {
        for col in row {
            print!("{}", col);
            if col == 'X' {
                total += 1;
            }
        }
        println!("");
    }

    for row in map1 {
        for col in row {
            print!("{}", col);
        }
        println!("");
    }

    println!("Total {} {}", total, options);
    Ok(())
}