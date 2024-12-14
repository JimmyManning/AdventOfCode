use std::fs::File;
use std::io::Read;
fn main() -> std::io::Result<()> {
    let mut file = File::open("input.txt")?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    let mut level: Vec<i32> = vec!();
    let mut levels: Vec<Vec<i32>> = vec!();
    let mut values: Vec<i32> = vec!();
    let mut x = contents.split("(");
    let mut total = 0;
    let mut mulenable = 1;
    loop {
        let y = x.next();


        match y {
            Some(y) => {
                if y.len() >= 2 && y[y.len()-2..y.len()].eq("do") {
                    mulenable = 1;
                    println!("GOT Do");
                }
                else if y.len() >= 5 && y[y.len()-5..y.len()].eq("don't") {
                    mulenable = 0;
                    println!("GOT Don't");
                }
                else if y.len() >= 3 && y[y.len()-3..y.len()].eq("mul") && mulenable == 1 {
                    let mut temp = x.clone();
                    //if temp.expect("ERROR").len() > 0 //&& !temp.unwrap()[0..0].eq(")")
                    {
                //let mut z = temp.expect("ERROR").split(")");
                //loop
                //{ 
                    let a = &temp.next();
                    println!("{:?}", a);
                    match a {
                        Some(a) => {
                            let mut ab = a.split(",");
                            let mut mults: Vec<i32> = vec!();
                            let mut error = 0;
                            let mut foundend = 0;
                            loop
                            {
                                let mut aa = ab.next().clone();
                                match aa {
                                    Some(mut aa) => {
                                        if aa.contains(')') && mults.len() == 1 {
                                            aa = &aa[0..aa.find(')').expect("ERROR")];
                                            foundend = 1;
                                        }
                                        let val : i32 = match aa.parse::<i32>()
                                        {
                                            Ok(n) => {
                                            n
                                            },
                                            Err(_) => {
                                                error = 1;
                                                break;
                                            }
                                        };
                                        mults.push(val);
                                        if foundend == 1 {
                                            break;
                                        }
                                    },
                                    None => {
                                        break;
                                    }
                                }

                            }
                            if mults.len() == 2 && error == 0 && foundend == 1
                            {
                                total += mults[0] * mults[1];
                                values.push(mults[0]);
                                values.push(mults[1]);
                                println!(" got {} {}", mults[0], mults[1]);
                            }
                            else
                            {
                                println!("NOPE");
                            }
                        },

                        None => {
                            //break;
                        }
                            
                            
                    }
                }
            }

            },
            None => {
                break;
            }
        }
    }
   
    println!("totals {}", total);
    Ok(())
}