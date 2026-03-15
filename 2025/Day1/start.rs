use std::fs::File;
use std::io::Read;
use std::collections::HashMap;
fn main() -> std::io::Result<()> {
    let mut file = File::open("input.txt")?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    let mut x = contents.split("\n");
    let mut total = 0;
    
    let mut value: i32 = 50;
    for val in x {
        if val.len() > 1{
            let mut mult: i32 = 1;
            let mut num = &val[1..];
            let mut i32num: i32 = num.parse::<i32>().expect("Failed to parse string to i32");
            if val.chars().nth(0).unwrap() == 'R' {
                mult = 1;
            }
            else if val.chars().nth(0).unwrap() == 'L'
            {
                mult = -1;
            }
            
    
            let mut start = val;
            //value += i32num * mult;

            if mult == -1 {
                while i32num > 0
                {
                    if value == 0
                    {
                        total += 1;
                        value = 99;
                    }
                    else {
                        value -=1;
                    }

                    i32num -= 1;
                }
            } 
            else 
            {
                while i32num > 0
                {   if (value == 0)
                    {
                        total += 1;
                    }
                    if value == 99
                    {
                        //total += 1;
                        value = 0;
                    }
                    else {
                        value += 1;
                    }

                    i32num -= 1;
                }
            }
            //if value == 0
            //{
            //    total += 1;
            //}
            // if value == 0 {
            //     total += 1;
            // }
            // while value < 0 {
            //     value += 100;
            //     //total += 1;
            // }
            // while value > 99 {
            //     value -= 100;
            //     //total += 1;
            // }

            println!("Total {} val {} {}", i32num * mult, value, total);
        }
    }

    println!("Total {} val {}", total, value);
    Ok(())
}