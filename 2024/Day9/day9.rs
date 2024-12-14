use std::fs::File;
use std::io::Read;
use std::collections::HashMap;
fn main() -> std::io::Result<()> {
    let mut file = File::open("puzzle.txt")?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    let mut x = contents.split("\n");
    let mut total : u64 = 0;
    let mut disk : Vec<i32> = vec!();
    loop {
        let line = x.next();
        match line {
            Some(line) => {
                let mut filenum : u32 = 0;
                let mut file : bool = true;
                for i in line.chars() {
                    let num = (i as u8 - '0' as u8) as u8;
                    for ii in 0..num {
                        if file {
                            disk.push(filenum as i32)
                        } 
                        else {
                            disk.push(-1);
                        }
                    }
                    if file {
                        filenum += 1;
                    }
                    file = !file;
                }

            },
            None => {
                break;
            }
        }
    }

    for i in 0..disk.len() {
        print!("{} ", disk[i]);
    }
    println!();
    let mut movefileattempt : HashMap<u32, bool> = Default::default();
    let mut curfile = 0;
    let mut filesize = 0;
    for mut i in (0..disk.len()).rev() {
        //if disk[i] != -1 {
            if filesize == 0 {
                if disk[i] != -1 { 
                    curfile = disk[i];
                }
            }
            if disk[i] != curfile && filesize != 0 && *movefileattempt.entry(disk[i-1] as u32).or_insert_with(|| false) == false {
                *movefileattempt.entry(disk[i-1] as u32).or_insert_with(|| true) = true;
                for mut ii in 0..disk.len() {
                    if i == ii {
                        i = 0;
                        break;
                    }
                    if disk[ii] == -1 {
                        let mut availsize = 0;
                        for mut iii in ii..disk.len() {
                            if disk[iii] == -1 {
                                availsize += 1;
                            }
                            else {
                                break;
                            }
                            if availsize == filesize
                            {
                                for iiii in 0..filesize {
                                    //println!("movind {} to {}  {} {} {} {}", i + iiii, ii + iiii, filesize, disk[i + filesize - iiii], iiii, availsize);
                                    disk[ii + iiii] = disk[i + filesize - iiii];
                                    disk[i + filesize - iiii] = -1;

                                    
                                }
                                ii = disk.len() + 1;
                                iii = disk.len() + 1;
                                break;
                            }
                        }
                        if availsize == filesize {
                            break;
                        }
                    }
                }
                if disk[i] != -1 {
                    filesize = 1;
                    curfile = disk[i]; 
                }
                else {
                    filesize = 0;
                }
            }
            else {
                if disk[i] != -1 {
                filesize += 1;
                }
            }
        //}
    }
    println!();

    for i in 0..disk.len() {
        print!("{} ", disk[i]);
    }

    for i in 0..disk.len() {
        if disk[i] == -1 {

        } 
        else {
        total += disk[i] as u64 * i as u64;
        }
    }
    

    println!("Total {} 10708206963185 6368258855761 6344759806413 6344830702833", total);
    Ok(())
}