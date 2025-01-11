use std::fs::File;
use std::io::Read;
use std::collections::HashMap;

fn move_blocks(grid : &mut Vec<Vec<char>>, mut start : (usize, usize), dir : (i32, i32)) -> bool
{
    let mut return_val : bool = false;
    if start.0 as i32 + dir.0 < grid[0].len() as i32 && start.0 as i32 + dir.0 >= 0 && start.1 as i32 + dir.1 < grid.len() as i32 && start.1 as i32 + dir.1 >= 0 
    {
        if grid[(start.1 as i32 + dir.1) as usize][(start.0 as i32 + dir.0) as usize] == 'O' {
            println!(" found a block");
            start.0 = (start.0 as i32 + dir.0) as usize;
            start.1 = (start.1 as i32 + dir.1) as usize;
            if (move_blocks(grid, start, dir))
            {
                grid[start.1][start.0] = 'O';
            }
        }
        else if grid[(start.1 as i32 + dir.1) as usize][(start.0 as i32 + dir.0) as usize] == '.' {
            return true;
        }
    }
    return return_val;
}

fn move_robot(grid : &mut Vec<Vec<char>>, start : &mut (usize, usize), dir : (i32, i32)) -> ()
{
    if (start.0 as i32 + dir.0) < grid[0].len() as i32 && (start.0 as i32 + dir.0) >= 0 && (start.1 as i32 + dir.1) < grid.len() as i32 && (start.1 as i32 + dir.1) >= 0 
    {
        if grid[(start.1 as i32 + dir.1) as usize][(start.0 as i32 + dir.0) as usize] == '.' {
            
            grid[start.1][start.0] = '.';
            start.0 = (start.0 as i32 + dir.0) as usize;
            start.1 = (start.1 as i32 + dir.1) as usize;
            println!(" Free shifting {:?}", start);
            grid[start.1][start.0] = '@';
        }
        else if grid[(start.1 as i32 + dir.1) as usize][(start.0 as i32 + dir.0) as usize] == 'O' { 
            println!("found an O");
            if move_blocks(grid, *start, dir)
            {
                grid[start.1][start.0] = '.';
                start.0 = (start.0 as i32 + dir.0) as usize;
                start.1 = (start.1 as i32 + dir.1) as usize;
                grid[start.1][start.0] = '@';
            }
        }
    }
    else 
    {
        println!(" no posible {:?} {:?}", start, dir);
    } 
}

fn main() -> std::io::Result<()> {
    let mut file = File::open("tmp.txt")?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    let mut x = contents.split("\n");
    let mut total = 0;
    let mut grid : Vec<Vec<char>> = vec!();
    let mut moves : Vec<Vec<char>> = vec!();
    let mut gridfilled : bool = false;
    let mut start : (usize, usize) = (0, 0);
    let mut rownum = 0;

    loop {
        let line = x.next();
        match line {
            Some(line) => {
                let mut row : Vec<char> = vec!();
                let mut colnum = 0;
                for col in line.chars() {
                    if col == '@' {
                        start.0 = colnum;
                        start.1 = rownum;
                    }
                    row.push(col);
                    colnum += 1;
                }
                if row.len() == 0 {
                    gridfilled = true;
                }
                else {
                    if gridfilled {
                        moves.push(row);
                    }
                    else {
                        grid.push(row);
                    }
                }
               
            },
            None => {
                break;
            }
        }
        if !gridfilled { 
            rownum += 1;
        }
    }
    let x = start.0;
    let y = start.1;

println!("Total {} {:?}", total, start);
    for i in 0..moves.len() {
        for ii in 0..moves[i].len() {
            let newx : i32 = 0;
            let newy : i32 = 0;
            if moves[i][ii] == '<' {
                move_robot(&mut grid, &mut start, (-1, 0));
            }
            else if moves[i][ii] == '>' {
                move_robot(&mut grid, &mut start, (1, 0));
            }
            else if moves[i][ii] == '^' {
                println!(" trying up");
                move_robot(&mut grid, &mut start, (0, -1));
            }
            else if moves[i][ii] == 'v' {
                move_robot(&mut grid, &mut start, (0, 1));
            }
            println!("--------{}-------------{:?}", moves[i][ii], start);
            for row in &grid {
                for col in row {
                    print!("{}", col);
                }
                println!();
            }

        }
    }
    
    for row in grid {
        for col in row {
            print!("{}", col);
        }
        println!();
    }
println!("--------------------");
    for row in moves {
        for col in row {
            print!("{}", col);
        }
        println!();
    }
    println!("Total {} {:?}", total, start);
    Ok(())
}