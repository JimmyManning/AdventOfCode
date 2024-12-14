use std::fs::File;
use std::io::Read;
use std::collections::HashMap;
use sscanf::sscanf;
use std::thread::sleep;
use core::time::Duration;
fn main() -> std::io::Result<()> {
    let mut file = File::open("input.txt")?;

    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    let mut x = contents.split("\n");
    let mut total = 0;
    let mut gridrows = 103;
    let mut gridcols = 101;
    let mut gaurds : Vec<((i32, i32),(i32, i32))> = vec!();

    loop {
        let mut line = x.next();
        match line {
            Some(line) => {
                let parsed = sscanf!(line, "p={i32},{i32} v={i32},{i32}");
                match parsed {
                    Ok(parsed) => {
                        println!("{} {} {} {}", parsed.0, parsed.1, parsed.2, parsed.3);
                        let gaurd : ((i32, i32),(i32, i32)) = ((parsed.0, parsed.1), (parsed.2, parsed.3));
                        gaurds.push(gaurd);
                    },
                    Error => {
                        break;
                    }
                }
            },
            None => {
                break;
            }
        }
    }

    for i in 0..100000 {


        for x in 0..gaurds.len() {
            let mut newx = 0;
            let mut newy = 0;
            let oldx = gaurds[x].0.0;
            let oldy = gaurds[x].0.1;
            let velx = gaurds[x].1.0;
            let vely = gaurds[x].1.1;

            if (oldx + velx) >= gridcols {
                newx = oldx + velx - gridcols;
                //print!("1");
            }   
            else if (oldx + velx) < 0 {
                newx = gridcols + oldx + velx;
                //print!("2");
            }
            else 
            {
                newx = oldx + velx;
                //print!("3");
            }

            if (oldy + vely) >= gridrows {
                newy = oldy + vely - gridrows;
                //print!("4");
            }
            else if (oldy + vely) < 0 {
                newy = gridrows + oldy + vely;
                //print!("5");
            }
            else 
            {
                newy = oldy + vely;
                //print!("6");
            }

            gaurds[x].0.0 = newx;
            gaurds[x].0.1 = newy;
            //println!("{:?} {:?}", gaurds[x].0, gaurds[x].1);
        }
        let mut middleset : bool = false;
        for x in 0..gaurds.len()
        {
            if gaurds[x].0.1 == 0 && gaurds[x].0.0 == (gridrows-1)/2 {
                middleset = true;
            }
        }
        if i == 7411 {
            let mut check : bool = false;
            for y in 0..gridrows {
                let mut gaurdsinarow = 0;
                for x in 0..gridcols {
                    let mut numgaurds = 0;
                    for gaurd in &gaurds {
                        if gaurd.0.0 == x && gaurd.0.1 == y {
                            numgaurds += 1;
                            
                        }
                        else {
                            
                        }
                    }
                    if numgaurds > 0 {
                        //print!("{}", 1);
                        gaurdsinarow += 1;
                    }
                    else {
                        if gaurdsinarow > 6 {
                            println!("{} check ", i);
                            check = true;
                        }
                        gaurdsinarow = 0;
                        //print!(".");
                    }
                }
                //println!();
            }
            println!("{}", i);
            if check {
                for y in 0..gridrows {
                    let mut gaurdsinarow = 0;
                    for x in 0..gridcols {
                        let mut numgaurds = 0;
                        for gaurd in &gaurds {
                            if gaurd.0.0 == x && gaurd.0.1 == y {
                                numgaurds += 1;
                                
                            }
                            else {
                                
                            }
                        }
                        if numgaurds > 0 {
                            print!("{}", 1);
                        }
                        else {
                            print!(".");
                        }
                    }
                    println!();
                }
                println!("{}", i);
            }
        }
    }

    let mut inone = 0;
    let mut intwo = 0;
    let mut inthree = 0;
    let mut infour = 0;

    // for i in 0..gaurds.len() {
    //     if gaurds[i].0.0 > gridcols/2 {
    //         if gaurds[i].0.1 >= gridrows/2 {
    //             inthree += 1;
    //         }
    //         {
    //             intwo += 1;
    //         }
    //     }
    //     else {
    //         if gaurds[i].0.1 >= gridrows/2 {
    //             infour += 1;
    //         }
    //         else 
    //         {
    //             inone += 1;
    //         }
    //     }
    // }
println!("---------------");
        for y in 0..gridrows {
            for x in 0..gridcols {
                let mut numgaurds = 0;
                for gaurd in &gaurds {
                    if gaurd.0.0 == x && gaurd.0.1 == y {
                        numgaurds += 1;
                    }
                }
                if numgaurds > 0 {
                    print!("{}", numgaurds);
                    if x > (gridcols-1)/2 {
                        if y > (gridrows-1)/2 {
                            inthree += numgaurds;
                        }
                        else if y == (gridrows-1)/2 {
                            println!("not {} {}", x, y);
                        }
                        else {
                            intwo += numgaurds;
                        }
                    }
                    else if x == (gridcols-1)/2{
                        println!("not {} {}", x, y);
                    }
                    else {
                        if y > (gridrows-1)/2 {
                            infour += numgaurds;
                        }
                        else if y == (gridrows-1)/2 {
                            println!("not {} {}", x, y);
                        }
                        else {
                            inone += numgaurds;
                        }
                    }
                }
                else {
                    print!(".");
                }
            }
            println!();
        }
    
    println!("{} {} {} {} {}", inone, intwo, infour, inthree, inone * intwo * inthree * infour);

    println!("Total {}", total);
    Ok(())
}