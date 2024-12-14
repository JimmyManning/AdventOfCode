use std::fs::File;
use std::io::Read;
use std::collections::HashMap;
fn main() -> std::io::Result<()> {
    let mut file = File::open("input.txt")?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    let mut x = contents.split("\n");
    let mut total = 0;
    let mut totals : Vec<u64> = vec!();
    let mut values : Vec<Vec<u64>> = vec!();
    
    loop {
        let mut line = x.next();
        match line
        {
            Some(line) => {
                let mut vals = line.split(" ");
                let mut curvals : Vec<u64> = vec!();
                loop {
                    let aa = vals.next();
                    match aa {
                        Some(mut aa) => {
                            let mut is_total : bool = false;
                            if aa.as_bytes()[aa.len() -1] == b':' {
                                is_total = true;
                                aa = &aa[0..aa.len()-1];
                            }
                            let val : u64 = match aa.parse::<u64>()
                            {
                                Ok(n) => {
                                n
                                },
                                Err(_) => {
                                    break;
                                }
                            };
                            if is_total {
                                totals.push(val);
                            }
                            else {
                                curvals.push(val);
                            }
                        },
                        None => {
                            break;
                        }
                    }
                }
                values.push(curvals);
            },
            None => {
                break;
            }

        }
    }

    for i in 0..totals.len() {
        println!("|{}", i);
        //for ops in 0..((u64::pow(2, ((values[i].len() -1) as u32)) as usize)) {
        let mut ops : Vec<i32> = vec!();
        for x in 0..values[i].len() -1 {
            ops.push(0);
        }
        loop {
            let mut curtotal : u64 = 0;
            for x in 0..values[i].len() -1 {
                if ops[x] == 0 {
                    if x == 0 {
                    curtotal += values[i][x] * values[i][x+1]
                    } else {
                        curtotal *= values[i][x+1]
                    }
                }
                else if ops[x] == 1 {
                    if x == 0 {
                        curtotal += values[i][x] + values[i][x+1]
                        } else {
                            curtotal += values[i][x+1]
                        }
                }
                else if ops[x] == 2 {
                    let s: String = values[i][x+1].to_string();
                    if x == 0 {
                        curtotal += (values[i][x] * u64::pow(10, s.len() as u32)) + values[i][x+1]
                    } else {
                        curtotal = (curtotal * u64::pow(10, s.len() as u32)) + values[i][x+1]
                    }
                }
            }

            print!("--{} {} {} = ", ops.len(), totals[i], curtotal);
            for x in 0..values[i].len() -1 {
                if x == 0 {
                    print!("{}", values[i][x]);
                }
                if ops[x] == 0 {
                    print!(" * ");
                }
                else if ops[x] == 1 { 
                    print!(" + ");
                }
                else if ops[x] == 2 { 
                    print!(" || ");
                }
                print!("{}", values[i][x+1]);
            }
            println!();
            if curtotal == totals[i] {
                println!(" {} Is Possible", i);
                total += totals[i];
                break;
            }
            let mut opindex = 0;
            loop {
                if opindex < ops.len() {
                if (ops[opindex] + 1) == 3 {
                    ops[opindex] = 0;
                    opindex += 1;
                } else {
                    ops[opindex] += 1;
                    break;
                }
                } else {
                    break;
                }
            }
            if opindex >= ops.len() {
                break;
            }
        }
    }

    println!("Total {}", total);
    Ok(())
}