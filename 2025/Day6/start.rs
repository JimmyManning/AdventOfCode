use std::fs::File;
use std::io::Read;
use std::collections::HashMap;
use std::convert::TryInto;
fn main() -> std::io::Result<()> {
    let mut file = File::open("input.txt")?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    let mut x = contents.split("\n");
    let mut total : u64 = 0;
    let mut nums : Vec<Vec<u64>> = Vec::new();
    let mut ops : Vec<char> = Vec::new();
    let mut opRange : Vec<(usize,usize)> = Vec::new();
    let mut rows : Vec<String> = Vec::new();
    for row in x {
        rows.push(row.to_string());
        let mut rowv : Vec<u64> = Vec::new();
        if row.chars().nth(0).unwrap() >= '0' && row.chars().nth(0).unwrap() <= '9' || row.chars().nth(0).unwrap() == ' ' {
            let mut columns = row.split(" ");
            for column in columns {
                if column.len() > 0 {
                println!("{}", column);
                let mut uval32: u64 = column.parse::<u64>().expect("Failed to parse string to u64");
                rowv.push(uval32);
                }

            }
            nums.push(rowv);
        }
        else {
            //let mut columns = row.split(" ");
            let mut opstart : usize = 0;
            let mut opend : usize = 0;
            for i in 0..row.len() {
                if row.chars().nth(i).unwrap() != ' ' {
                    println!("{}", row.chars().nth(i).unwrap());
                    ops.push(row.chars().nth(i).unwrap());
                    if i == 0 {

                    }
                    else {
                        opRange.push((opstart, (i - 2).try_into().unwrap()));
                        opstart = i;
                    }
                    
                }
            }
            opRange.push((opstart, (row.len() -1).try_into().unwrap()));
        }
    }

    for i in &opRange {
        println!("{}-{}", i.0, i.1);
    }

    for i in 0..ops.len() {
        if ops[i] == '+' {
            let mut addTotal = 0;
            for x in opRange[i].0..=opRange[i].1 {
                let mut my_string = String::from("");
                for y in 0..nums.len() {
                    if rows[y].chars().nth(x).unwrap() != ' ' {
                        my_string.push(rows[y].chars().nth(x).unwrap());
                    }
                }
                let mut uval32: u64 = my_string.parse::<u64>().expect("Failed to parse string to u64");
                addTotal += uval32;
                println!("{}", my_string);
            }
            println!("add total {}", addTotal);
            total += addTotal;
        }
        if ops[i] == '*' {
            let mut multTotal : u64 = 1;
            for x in opRange[i].0..=opRange[i].1 {
                let mut my_string = String::from("");
                for y in 0..nums.len() {
                    if rows[y].chars().nth(x).unwrap() != ' ' {
                        my_string.push(rows[y].chars().nth(x).unwrap());
                    }
                }
                let mut uval32: u64 = my_string.parse::<u64>().expect("Failed to parse string to u64");
                multTotal *= uval32;
                println!("{}", my_string);
            }
            println!("mult total {}", multTotal);
            total += multTotal;
        }
    }
    

    println!("Total {}", total);
    Ok(())
}