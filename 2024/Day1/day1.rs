use std::fs::File;
use std::io::Read;
use std::collections::HashMap;
fn main() -> std::io::Result<()> {
    let mut file = File::open("input.txt")?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    let mut left_list: Vec<i32> = vec!();
    let mut right_list: HashMap<i32, i32> = HashMap::new();
    let mut x = contents.split("\n");

    loop {
        let y = x.next();

        match y {
            Some(y) => {
                if let [Ok(aa), Ok(aaa)] = &y.split("   ")
                                 .map(|y| y.parse::<i32>())
                                 .collect::<Vec<_>>()[..] {
                                                            *right_list.entry(*aaa).or_insert(0) += 1;
                                                            left_list.push(*aa);
                }

            },
            None => {
                break;
            }
        }
    }

    let mut total = 0;
    for key in left_list.clone().into_iter() {
        let cur = (key) * *right_list.entry(key).or_insert(0);
        println!(" left {} right  {} {}", key, right_list.entry(key).or_insert(0), cur);
        total += cur;
        }
    

    println!("Total {}", total);
    Ok(())
}