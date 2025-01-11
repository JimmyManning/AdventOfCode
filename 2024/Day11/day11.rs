use std::fs::File;
use std::io::Read;
use std::collections::HashMap;
fn main() -> std::io::Result<()> {
    let mut file = File::open("input.txt")?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    let mut x = contents.split("\n");
    let mut total = 0;
    let mut stones : Vec<u64> = vec!();

    loop {
        let mut line = x.next();
        match line {
            Some(line) => {
                let mut nums = line.split(" ");
                loop {
                    let num = nums.next();
                    match (num) {
                        Some(num) => {
                            let mut is_total : bool = false;
                            let val : u64 = match num.parse::<u64>()
                            {
                                Ok(n) => {
                                n
                                },
                                Err(_) => {
                                    break;
                                }
                            };
                            stones.push(val);
                        },
                        Error => {
                            break;
                        }
                    }
                }
            }, 
            None => {
                break;
            }
        }
    }

    for i in 0..75 {
        let mut newstones : Vec<u64> = vec!();

        for mut stone in &mut stones {
            if *stone == 0 {
                *stone = 1;
                //newstones.push(1);
            }
            else if (stone.to_string().len() % 2 == 0) {
                let left : u64 = match stone.to_string()[0..stone.to_string().len()/2].parse::<u64>()
                {
                    Ok(n) => {
                    n
                    },
                    Err(_) => {
                        break;
                    }
                };

                let right : u64 = match stone.to_string()[stone.to_string().len()/2..stone.to_string().len()].parse::<u64>()
                {
                    Ok(n) => {
                    n
                    },
                    Err(_) => {
                        break;
                    }
                };
                *stone = left;
            
                newstones.push(right);
                //newstones.push(right);
            }
            else {
                *stone *= 2024;
                //newstones.push(stone * 2024);
            }
        }
        for newstone in newstones {
            stones.push(newstone);
        }
        //stones = newstones;
        //for stone in &stones {
            //print!(" {} ", stone);
        //}
        println!("{} {}", i, stones.len());
    }

    

    println!("Total {}", stones.len());
    Ok(())
}