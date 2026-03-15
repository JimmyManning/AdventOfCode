use std::fs::File;
use std::io::Read;
use std::collections::HashMap;
fn checkElem(map: &Vec<Vec<char>>, x : usize, y : usize) -> u64 {
    let mut countnear = 0;
    let mut movable = 0;
    if map[y][x] == '@' {
        if x == 0 {
            if map[y][x+1] == '@' {
                countnear += 1;
            }

            if y == 0 {
                if map[y+1][x] == '@' {
                    countnear += 1;
                }
                if map[y+1][x+1] == '@' {
                countnear += 1;
                }
            }
            else {
                if map[y-1][x] == '@' {
                    countnear += 1;
                }
                if map[y-1][x+1] == '@' {
                    countnear += 1;
                }
                if y == map.len() - 1 {

                }
                else {
                    if map[y+1][x] == '@' {
                        countnear += 1;
                    }
                    if map[y+1][x+1] == '@' {
                        countnear += 1;
                    }
                }
            }
        }
        else {
            if map[y][x-1] == '@' {
                countnear += 1;
            }

            if y == 0 {
                if map[y+1][x] == '@' {
                    countnear += 1;
                }
                if map[y+1][x-1] == '@' {
                    countnear += 1;
                }

                if x == map[0].len() -1 {

                }
                else {
                    if map[y][x+1] == '@' {
                        countnear += 1;
                    }
                    if map[y+1][x+1] == '@' {
                        countnear += 1;
                    }
                }
            }
            else {
                if map[y-1][x] == '@' {
                    countnear += 1;
                }
                if map[y-1][x-1] == '@' {
                    countnear += 1;
                }

                if x == map[0].len() -1 {

                }
                else {
                    if map[y-1][x+1] == '@' {
                        countnear += 1;
                    }
                    if map[y][x+1] == '@' {
                        countnear += 1;
                    }                    
                }
                if y == map.len() - 1 {

                }
                else {
                    if map[y+1][x] == '@' {
                        countnear += 1;
                    }
                    if map[y+1][x-1] == '@' {
                        countnear += 1;
                    }
                    if x == map[0].len() -1 {

                    }
                    else {
                        if map[y+1][x+1] == '@' {
                            countnear += 1;
                        }
                    }                    
                }
            }


        }

        if countnear < 4 {
            movable = 1;
        }
    }

    return movable;
}


fn main() -> std::io::Result<()> {
    let mut file = File::open("input.txt")?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    let mut mapOfPaper: Vec<Vec<char>> = Vec::new();
    let mut x = contents.split("\n");
    let mut total = 0;
    for row in x {
        let mut vecrow = vec!();
        for col in row.chars() {
            vecrow.push(col);
        }
        mapOfPaper.push(vecrow);
    }
let mut mapOfPaper2: Vec<Vec<char>> = mapOfPaper.clone();
let mut removedThisPass = 0;
loop  {
    removedThisPass = 0;
    for y in 0..mapOfPaper.len() {
        for x in 0..mapOfPaper[y].len() {
            let mut found = checkElem(&mapOfPaper, x, y);
            if found == 1 {
                total += 1;
                removedThisPass += 1;
                mapOfPaper2[y][x] = 'x';
            }
        }
    }
    if removedThisPass == 0 {
        break;
    }
    mapOfPaper = mapOfPaper2.clone();
}
    
    for row in mapOfPaper {
        for col in row {
            print!("{}", col);
        }
        println!("");
    }
    println!("");
        for row in mapOfPaper2 {
        for col in row {
            print!("{}", col);
        }
        println!("");
    }


    println!("Total {}", total);
    Ok(())
}