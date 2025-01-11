use std::fs::File;
use std::io::Read;
use std::collections::HashMap;

fn traverse_digit(mut grid : Vec<Vec<char>>, start : char, end : char, path : Vec<char>) -> Vec<char> {
    return path;
}

fn main() -> std::io::Result<()> {
    let mut file = File::open("puzzle.txt")?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    let mut x = contents.split("\n");
    let mut total = 0;
    let mut codes : Vec<Vec<char>>  = vec!();

    loop {
        let mut line = x.next();
        match line {
                
                Some(line) => {
                    let mut code : Vec<char> = vec!();
                    for digit in line.chars() {
                        code.push(digit);
                    }
                    codes.push(code);
                },
                None => {
                    break;
            }
        }
    }
    let mut keypad : Vec<Vec<char>> = [
        ['7', '8','9'].to_vec(),
        ['4', '5', '5'].to_vec(),
        ['1', '2', '3'].to_vec(),
        [' ', '0', 'A'].to_vec(),
    ].to_vec();
    let start : char = 'A';
    let mut path : Vec<char> = vec!();
    for code in codes {
        for digit in code {
            print!("{} ", digit);
            path = traverse_digit(keypad.clone(), start, digit, path);
        }
        println!();
    }
    

    println!("Total {}", total);
    Ok(())
}