use std::fs::File;
use std::io::Read;
use std::collections::HashMap;
use std::convert::TryInto;
fn factorial(battery: &str, index : usize, digit : i32, curVal : u64, highestVal : u64) -> u64 {
    if digit < 0 {
        return curVal;
    }
    let mut highestValue = highestVal;
    if  highestValue == 0 {
        highestValue = curVal;
    }
    let mut udigit : usize = digit.try_into().unwrap();
    let mut max : usize = (battery.len() - udigit).try_into().unwrap();
    let mut highestIndexVal = 0;
    let mut highestIndex = 0;
    for i in index..max {
        let mut cur = battery.chars().nth(i.try_into().unwrap());
        let mut u64Cur: u64 = cur.expect("REASON").to_digit(10).expect("Failed to parse string to i32").into();
        
        let mut curValSum = 0;
        if u64Cur > highestIndexVal {
            highestIndexVal = u64Cur;
            highestIndex = i;
        }
    }

    let mut curValSum = 0;
    let digits : u64 = 10;
    if (highestIndexVal+1) * (digits.pow(digit.try_into().unwrap())) + curVal > highestValue {
        curValSum = factorial(battery, highestIndex+1, digit - 1, curVal + highestIndexVal * (digits.pow(digit.try_into().unwrap())), highestValue);
    }
    if curValSum > highestValue
    {
        highestValue = curValSum;
    }

    // for i in index..max {
    //     let mut cur = battery.chars().nth(i.try_into().unwrap());
    //     let mut u64Cur: u64 = cur.expect("REASON").to_digit(10).expect("Failed to parse string to i32").into();
    //     let digits : u64 = 10;
    //     let mut curValSum = 0;
    //     if (u64Cur+1) * (digits.pow(digit.try_into().unwrap())) + curVal > highestValue {
    //         curValSum = factorial(battery, i+1, digit - 1, curVal + u64Cur * (digits.pow(digit.try_into().unwrap())), highestValue);
    //     }
    //     if curValSum > highestValue
    //     {
    //         highestValue = curValSum;
    //     }
    // }
    return highestValue;
}

fn main() -> std::io::Result<()> {
    let mut file = File::open("input.txt")?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    let mut x = contents.split("\n");
    let mut total = 0;
    for val in x {
        let mut highestVal = 0;
        println!("battery {}", val);
        highestVal = factorial(val.clone(), 0, 11, 0, 0);
        // for i in 0..val.len()-12 {
        //     let mut curVal : u64 = 0;
        //     for j in i..i+12 {
        //         let mut cur = val.chars().nth(j);
        //         let mut u64Cur: u64 = cur.expect("REASON").to_digit(10).expect("Failed to parse string to i32").into();
        //         let digits : u64 = 10;
        //         let place : u32 = (11 - (j - i)).try_into().unwrap();
        //         println!("{} {} {} {}", u64Cur, place, (u64Cur * (digits.pow(place))), curVal);
        //         curVal += u64Cur * (digits.pow(place));

        //     }
        //     println!("{}", curVal);
        //     if curVal > highestVal {
        //         highestVal = curVal;
        //     }
        // }
        println!("{}", highestVal);
        total += highestVal;

    }
    

    println!("Total {}", total);
    Ok(())
}