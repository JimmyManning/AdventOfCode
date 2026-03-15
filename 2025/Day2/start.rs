use std::fs::File;
use std::io::Read;
use std::collections::HashMap;
fn main() -> std::io::Result<()> {
    let mut file = File::open("input.txt")?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    let mut x = contents.split(",");
    let mut total = 0;

    for val in x {
        let (low, high) = val.split_once('-')
                             .expect("String should contain a dash");
        //println!("{} - {}", low, high);

        let mut i64high: i64 = high.parse::<i64>().expect("Failed to parse string to i64");
        let mut i64low: i64 = low.parse::<i64>().expect("Failed to parse string to i64"); 

        for num in i64low..=i64high {
            //println!("{}", num);

            let strnum = num.to_string();
            let mut numf = 0;
            for i in 1..=strnum.len() / 2 {
                let snip = &strnum[0..i];
                if strnum.len() % i == 0 {
                for x in 0..strnum.len()/snip.len() {
                    if snip == &strnum[i*x..i*x+snip.len()] {
                        numf += 1;
                    }
                    else 
                    {
                        numf = 0;
                        break;
                    }
                }
                }
                if numf != 0 {
                    total += num;
                    println!("found {}", num);
                    break;

                }
            }

            // if strnum.len() % 2 == 0 {
            //     let left = &strnum[0..strnum.len()/2];
            //     let right = &strnum[strnum.len()/2..];
            //     if left == right {
            //         println!("Found one {}", num);
            //         total += num;
            //     }
            // }
        }
    }
    

    println!("Total {}", total);
    Ok(())
}