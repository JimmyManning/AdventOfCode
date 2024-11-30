use std::fs::File;
use std::io::prelude::*;
use std::collections::HashMap;

fn get_int(arg: &String, index: usize, _y: usize) -> (usize, u32) {
    let val: u32 = 0;
    match arg[index..index+3].to_string().parse::<u32>() {
        Ok(x) => {
            return (index, x);
        },
        Err(_x) => {

        }
    }
    match arg[index-1..index+2].to_string().parse::<u32>() {
        Ok(x) => {
            return (index-1, x);
        },
        Err(_x) => {
        }
    }
    match arg[index-2..index+1].to_string().parse::<u32>() {
        Ok(x) => {
            return (index-2, x);
        },
        Err(_x) => {
        }
    }
    match arg[index..index+2].to_string().parse::<u32>() {
        Ok(x) => {
            return (index, x);
        },
        Err(_x) => {

        }
    }
    match arg[index-1..index+1].to_string().parse::<u32>() {
        Ok(x) => {
            return (index-1, x);
        },
        Err(_x) => {

        }
    }
    match (arg.clone().into_bytes()[index] as char).to_string().parse::<u32>() {
        Ok(x) => {
            return (index, x);
        },
        Err(_x) => {

        }
    }
    (index, val)
}

fn main() -> std::io::Result<()> {
    let mut file = File::open("puzzle.txt")?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    println!("{}", contents);

    let mut my_list: Vec<String> = vec![];
    let mut x = contents.split('\n');
    let mut part_graph: HashMap<usize, HashMap<usize, u32>> = HashMap::new();
    let mut index = 0;
    loop {
        let y = x.next();
        match y{
            Some(y) => {
                part_graph.insert(index, HashMap::new());
                index = index + 1;
                my_list.push(y.to_string());
            },
            None => {
                break;
            }
        }
    }
    println!("Start\n");
    
    let mut total: u32 = 0;
    let mut part_graph: HashMap<usize, HashMap<usize, u32>> = HashMap::new();
    for n in 0..my_list.len() {
        for y in 0..my_list[n].len() {
            let current_char: char = my_list[n].as_bytes()[y] as char;
            let gears: HashSet<u32> = HashSet::new();
            match current_char {
                '0'..='9' => {

                }
                '.' => {
                }
                _ => {
                    match my_list[n-1].as_bytes()[y] {
                        b'0'..=b'9' => {
                            let (col, val) = get_int(&my_list[n-1], y, n-1);
                            part_graph.entry(n-1).and_modify(|co| {co.entry(col).and_modify(|l| *l=val).or_insert(val);}).or_insert({let mut temp_map = HashMap::new(); temp_map.insert(col, val); temp_map});
                            gears.insert(v);
                        },
                        _ => {
                        }
                    }
                    match my_list[n-1].as_bytes()[y-1] {
                        b'0'..=b'9' => {
                            let (col, val) = get_int(&my_list[n-1], y-1, n-1);
                            part_graph.entry(n-1).and_modify(|co| {co.entry(col).and_modify(|l| *l=val).or_insert(val);}).or_insert({let mut temp_map = HashMap::new(); temp_map.insert(col, val); temp_map});
                            gears.insert(v);
                        },
                        _ => {
                        }
                    }
                    match my_list[n-1].as_bytes()[y+1] {
                        b'0'..=b'9' => {
                            let (col, val) = get_int(&my_list[n-1], y+1, n-1);
                            part_graph.entry(n-1).and_modify(|co| {co.entry(col).and_modify(|l| *l=val).or_insert(val);}).or_insert({let mut temp_map = HashMap::new(); temp_map.insert(col, val); temp_map});
                            gears.insert(v);
                        },
                        _ => {
                        }
                    }

                    match my_list[n+1].as_bytes()[y] {
                        b'0'..=b'9' => {
                            let (col, val) = get_int(&my_list[n+1], y, n+1);
                            part_graph.entry(n+1).and_modify(|co| {co.entry(col).and_modify(|l| *l=val).or_insert(val);}).or_insert({let mut temp_map = HashMap::new(); temp_map.insert(col, val); temp_map});
                            gears.insert(v);
                        },
                        _ => {
                        }
                    }
                    match my_list[n+1].as_bytes()[y-1] {
                        b'0'..=b'9' => {
                            let (col, val) = get_int(&my_list[n+1], y-1, n+1);
                            part_graph.entry(n+1).and_modify(|co| {co.entry(col).and_modify(|l| *l=val).or_insert(val);}).or_insert({let mut temp_map = HashMap::new(); temp_map.insert(col, val); temp_map});
                            gears.insert(v);
                        },
                        _ => {
                        }
                    }
                    match my_list[n+1].as_bytes()[y+1] {
                        b'0'..=b'9' => {
                            let (col, val) = get_int(&my_list[n+1], y+1, n+1);
                            part_graph.entry(n+1).and_modify(|co| {co.entry(col).and_modify(|l| *l=val).or_insert(val);}).or_insert({let mut temp_map = HashMap::new(); temp_map.insert(col, val); temp_map});
                            gears.insert(v);
                        },
                        _ => {
                        }
                    }

                    match my_list[n].as_bytes()[y-1] {
                        b'0'..=b'9' => {
                            let (col, val) = get_int(&my_list[n], y-1, n);
                            part_graph.entry(n).and_modify(|co| {co.entry(col).and_modify(|l| *l=val).or_insert(val);}).or_insert({let mut temp_map = HashMap::new(); temp_map.insert(col, val); temp_map});
                            gears.insert(v);
                        },
                        _ => {
                        }
                    }

                    match my_list[n].as_bytes()[y+1] {
                        b'0'..=b'9' => {
                            let (col, val) = get_int(&my_list[n], y+1, n);
                            part_graph.entry(n).and_modify(|co| {co.entry(col).and_modify(|l| *l=val).or_insert(val);}).or_insert({let mut temp_map = HashMap::new(); temp_map.insert(col, val); temp_map});
                            gears.insert(v);
                        },
                        _ => {
                        }
                    }
                   
                }
            }
        }
    }

    for (row, col) in &part_graph {
        print!("{} ", row);
        for (_c, val) in col {
            total += val;
            print!("{} ", val);
        }
        println!("\n");
    }
    
    for n in 0..my_list.len() {
        let mut y = 0;
        print!("{} ", n);
        while y < my_list[n].len() {
            match part_graph.get(&n) {
                Some(col) => {
                    match col.get(&y) {
                        Some(v) => {
                            for _i in 0..v.to_string().len() {
                                print!(".");
                                y = y + 1;
                            }
                        },

                        None => {
                            print!("{}", my_list[n].as_bytes()[y] as char);
                            y+= 1
                        }
                    }
                },
                None => {
                    print!("{}", my_list[n].as_bytes()[y] as char);
                    y += 1;
                }
            }
        }
        println!();
    }


    println!("Total: {}", total);
    Ok(())
}