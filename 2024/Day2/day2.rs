use std::fs::File;
use std::io::Read;
fn main() -> std::io::Result<()> {
    let mut file = File::open("input.txt")?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    let mut level: Vec<i32> = vec!();
    let mut levels: Vec<Vec<i32>> = vec!();
    let mut x = contents.split("\n");

    loop {
        let y = x.next();

        match y {
            Some(y) => {
                println!("level {}", y);
                let mut z = y.split(" ");
                loop
                { 
                    let aa = &z.next();
                    match aa {
                        Some(aa) => {
                            level.push(aa.parse::<i32>().unwrap());
                        },
                        None => {
                            break;
                        }
                    
                    }

                }
                levels.push(level.clone());
                level.clear();

            },
            None => {
                break;
            }
        }
    }
    let mut safereprts = 0;
    for i in levels {
        let mut up = -1;
        let mut val = -1;
        let mut good = 1;
        let mut skip = 0;
        let mut numberbad = 0;
        print!("VAL: ");
        for cur in &i
        {
            print!(" {}", cur);
        }
        print!("\n");
        let mut index = 0;
        for cur in &i
        {
            
            if val != -1
            {
                if up == -1 {
                    if val > *cur && (val - *cur) <=3 && val != *cur
                    {
                        up = 1;
                    }
                    else if val < *cur && ((*cur - val) <=3) && val != *cur
                    {
                        up = 0;
                    }
                    else
                    {
                        //good = 0;
                        
                        if ((val > i[2] && i[2] > i[3]) || (i[2] > val && i[3] > i[2])) && (val - i[2]).abs() <= 3
                        {
                            println!("tAKING VAK {} {} {} {}", val, cur, i[2], i[3]);
                            if val > i[2]
                            {
                                up = 1;
                            }
                            else 
                            {
                                up = 0;
                            }
                            skip = 1;
                        } 
                        else
                        {
                            println!("tAKING cur {} {} {} {}", val, cur, i[2], i[3]);
                            skip = 1; 
                            val = *cur;
                            if *cur > i[2]
                            {
                                up = 1;
                            }
                            else 
                            {
                                up = 0;
                            }
                        }
                    }
                }
                else
                {
                    if up == 0 && (val > *cur || (*cur - val) > 3 || val == *cur) {
                        println!("Nope2 {} {}", *cur, val);
                        //good = 0;
                        skip = 1;
                        if index+1 < i.len() && cur < &i[index+1]  
                        {
                            val = *cur;
                        }
                    }
                    else if up == 1 && (val < *cur || (val - *cur) > 3 || val == *cur){
                        println!("Nope1");
                        //good = 0;
                        skip = 1;
                        if index < i.len() && cur < &i[index+1]  
                        {
                            val = *cur;
                        }
                    }
                }

            }
            if skip == 0
            {
                val = *cur;
            }
            else
            {
                numberbad += 1;
                skip = 0;
                if numberbad > 1
                {
                    good = 0;

                    break;
                }
                
            }
            index += 1;
        }

        if good == 1 
        {
            safereprts += 1;
        }

    }
    println!("safe reports {} ", safereprts);
    Ok(())
}