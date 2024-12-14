use std::fs::File;
use std::io::Read;
use std::collections::HashMap;
fn main() -> std::io::Result<()> {
    let mut file = File::open("input.txt")?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    let mut orders : HashMap<i32, Vec<i32>> = Default::default();
    let mut x = contents.split("\n");
    let mut total = 0;
    let mut part_2_total = 0;
    loop {
        let val = x.next();
        match val {
            Some(val) => {
                let mut vals = val.split("|");
                let mut order : Vec<i32> = vec!();
                loop {
                    let aa = vals.next();
                    match aa {
                        Some(mut aa) => {
                            let val : i32 = match aa.parse::<i32>()
                            {
                                Ok(n) => {
                                n
                                },
                                Err(_) => {
                                    break;
                                }
                            };
                            println!("{}", val);
                            order.push(val);
                        },
                        None => {
                            break;
                        }
                    }
                }
                if order.len() == 0 {
                    break;
                }
                println!("adding {} {} to orders", order[0], order[1]);
                orders.entry(order[0]).or_insert_with(|| vec!()).push(order[1]);
                
            },
            None => {
                break;
            }
        }
    }

    let mut prints : Vec<Vec<i32>> = vec!();
    loop
    {
        let val = x.next();
        match val {
            Some(val) => {
                let mut vals = val.split(",");
                let mut pages_to_print : Vec<i32> = vec!();
                loop {
                    let aa = vals.next();
                    match aa {
                        Some(mut aa) => {
                            let val : i32 = match aa.parse::<i32>()
                            {
                                Ok(n) => {
                                n
                                },
                                Err(_) => {
                                    break;
                                }
                            };
                            println!("{}", val);
                            pages_to_print.push(val);
                        },
                        None => {
                            break;
                        }
                    }
                }
                if pages_to_print.len() == 0 {
                    break;
                }
                prints.push(pages_to_print);
            },
            None => {
                break;
            }
        }

    }

    for (before, after) in &orders {
        for oneafter in after {
            println!("{} {}", before, oneafter);
        }
    }

    for mut print in prints {
        let mut correct : bool = true;
         for mut i in 0..print.len() {
             print!("{},", print[i]);
         }
        println!("");
        let mut i = 0;
        loop {
            if (i == print.len()) {
            break;
            }
            //print!(" {} ", print[i]);
            let order = orders.get(&print[i]);
            match order {
                Some(order) => {
                    if i > 0 {
                        for ii in 0..i {
                            for after in order {
                                if print[ii] == *after {
                                    correct = false;
                                    let tmp = print[i];
                                    print[i] = print[ii];
                                    print[ii] = tmp;
                                    
                                    print!("( {} {} {} {}) ", print[i], print[ii], i, ii);
                                    for i in 0..print.len() {
                                        print!("{},", print[i]);
                                    }
                                    println!("");
                                    i = 0;

                                    continue;
                                }
                            }
                            if i == 0 {
                                break;
                            }
                        }
                    }
                },
                None => {

                }
            }
            i = i+1;
        }
        //print!("{} ", correct);
        if correct {
            total += print[print.len()/2];
        }
        else {
            part_2_total += print[print.len()/2];
            for mut i in 0..print.len() {
                print!("{},", print[i]);
            }
            println!("");
        }
       
        

        
    }
    

    println!("Total {}", total);
    println!("Total Part 2 {}", part_2_total);
    println!("4986 < x < 5276");
    Ok(())
}