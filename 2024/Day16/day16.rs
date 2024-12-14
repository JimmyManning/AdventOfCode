use std::fs::File;
use std::io::Read;
use std::collections::HashMap;
fn main() -> std::io::Result<()> {
    let mut file = File::open("puzzle.txt")?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    let mut x = contents.split("\n");
    let mut total = 0;
    

    println!("Total {}", total);
    Ok(())
}