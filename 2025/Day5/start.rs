use std::fs::File;
use std::io::Read;
use std::collections::HashMap;
use std::collections::HashSet;
fn main() -> std::io::Result<()> {
    let mut file = File::open("input.txt")?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    let mut x = contents.split("\n");
    let mut total = 0;
    let mut rangesFinished = 0;
    let mut ranges: Vec<(u64, u64)> = Vec::new();
    let mut secondTotal = 0;
    for row in x {
        println!("{}", row);
        if row.len() < 1 {
            rangesFinished = 1;
        }
        else {
            if rangesFinished == 0 {
                let (low, high) = row.split_once('-')
                                .expect("String should contain a dash");
                let mut u64high: u64 = high.parse::<u64>().expect("Failed to parse string to u64");
                let mut u64low: u64 = low.parse::<u64>().expect("Failed to parse string to u64"); 
                ranges.push((u64low, u64high));

            }
            else {
                let mut valToFind : u64 = row.parse::<u64>().expect("Failed to parse string to u64");
                let mut found = 0;
                for range in &ranges {
                    if valToFind >= range.0 && valToFind <= range.1 {
                        found = 1;
                        break;
                    }
                }
                if found == 1 {
                    println!("SPOILED {}", valToFind);
                    total += 1;
                }
            }
        }
    }

    println!("{}", ranges.len());
    //let mut fresh = HashSet::new();
    let mut index = 0;
    let mut rangeSizes : HashMap<u64, Vec<usize>>= HashMap::new();
    for range in &ranges {
        if let Some(val) = rangeSizes.get_mut(&(range.1 - range.0)) 
        { 
            val.push(index); 
        }
        else { 
            rangeSizes.insert(range.1 - range.0, Vec::new()); 
            if let Some(val) = rangeSizes.get_mut(&(range.1 - range.0)) { val.push(index);}
            };
        index += 1;
    }
    let mut num = 0;
    let mut sortedRanges = rangeSizes.keys().copied().collect::<Vec<_>>();
    sortedRanges.sort();
    let mut sortedIndex: Vec<usize> = Vec::new();
    let mut rangesSorted: Vec<(u64, u64)> = Vec::new();
    for i in sortedRanges.iter().rev() {
        println!("{}", i);
        for j in rangeSizes.get_mut(i).unwrap() {
            sortedIndex.push(*j);
            rangesSorted.push(ranges[*j]);
        }
    }

    for i in sortedIndex {
        println!("---{}", i);
        num += 1;
    }
    println!("{} {} {}", rangeSizes.len(), ranges.len(), num);
    

    let mut maxVal = ranges.len();
    let mut maxInner = maxVal;
    let mut i = 0;
    for k in 0..rangesSorted.len() {

        i += 1;
        if i > maxVal {
            break;
        }
        println!("{}", i);
        let mut subtract = 0;
        let mut currentRange = rangesSorted[k];
        println!("{}-{}", currentRange.0, currentRange.1);
        for j in 0..k {
            println!("k{} j{} i{}", k, j, i);
            //.   14.     18
            //  i  |-------|
            //.         16.    20
            // j         |-----| x
            if currentRange.0 <= rangesSorted[j].0 && currentRange.1 >= rangesSorted[j].0 {
                println!("Overlap1 {} {} {} {}", currentRange.0, currentRange.1, rangesSorted[j].0, rangesSorted[j].1);
                currentRange.1 = rangesSorted[j].0 - 1;
                println!("Overlap1- {} {} {} {}", currentRange.0, currentRange.1, rangesSorted[j].0, rangesSorted[j].1);
                //subtract = currentRange.1 - ranges[j].0;
            }

             //  i  |-------|
             // j     |--|.       x
             // Hoping becasue of sorting this doesn't exist. 
            // if currentRange.0 <= ranges[j].0 && currentRange.1 >= ranges[j].1  {
            //     println!("Overlap2 {} {} {} {}", currentRange.0, currentRange.1, ranges[j].0, ranges[j].1);
                
            //     ranges.push((ranges[j].1, currentRange.1));
            //     maxVal += 1;
            //     println!("max {}", maxVal);
                
            //     //subtract = ranges[j].1 - ranges[j].0;
            //     println!("split to {} {} and {} {}", ranges[j].1, currentRange.1, currentRange.0, ranges[j].0 - 1);
            //     currentRange.1 = ranges[j].0 - 1;
            // }

             //  i  |-------|
             // j|------------|
            if currentRange.0 >= rangesSorted[j].0 && rangesSorted[j].1 >= currentRange.1  {
                println!("Overlap3 {} {} {} {}", currentRange.0, currentRange.1, rangesSorted[j].0, rangesSorted[j].1);
                currentRange.0 = 0;
                currentRange.1 = 0;
                subtract = 1;
                break;
            }
             //.    12.     18
             //  i  |-------|
             //. 10.    14
             // j|------|
            if currentRange.0 >= rangesSorted[j].0 && rangesSorted[j].1 >= currentRange.0  {
                println!("Overlap4 {} {} {} {}", currentRange.0, currentRange.1, rangesSorted[j].0, rangesSorted[j].1);
                //subtract = ranges[j].1 - currentRange.0;
                currentRange.0 = rangesSorted[j].1 + 1;
                println!("Overlap4- {} {} {} {}", currentRange.0, currentRange.1, rangesSorted[j].0, rangesSorted[j].1);
            }
        }
        println!("Range - {} {} {}", currentRange.1, currentRange.0, currentRange.1 - currentRange.0 + 1);
        secondTotal += (currentRange.1 - currentRange.0) + 1 - subtract;


    }
    

    println!("Total {} {}", total, secondTotal);
    Ok(())
}