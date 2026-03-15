use std::fs::File;
use std::io::Read;
use std::collections::HashMap;
fn checkElem(map: &mut Vec<Vec<char>>, x : usize, y : usize, hashmap: &mut HashMap<(usize, usize), usize>) -> usize {
    let mut splits = 1;
    if y < map.len() {
        if map[y][x] == '^' {
            splits = 0;
            if x > 0 {
                splits = checkElem(map, x - 1, y + 1, hashmap);
            }
            if x < map[y].len() {
                splits += checkElem(map, x + 1, y + 1, hashmap);
            }
            hashmap.insert((x,y), splits);
            map[y][x] = '?' 
        }
        else if map[y][x] == '?' {
            splits = *hashmap.get(&(x,y)).unwrap();
        }
        else {
            splits = checkElem(map, x, y+1, hashmap);
        }
    }
    return splits;
}


fn main() -> std::io::Result<()> {
    let mut file = File::open("input.txt")?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    let mut map : Vec<Vec<char>> = Vec::new();
    let mut x = contents.split("\n");
    let mut start = (0, 0);
    let mut xcord = 0;
    let mut ycord = 0;

    for row in x {
        let mut rowv : Vec<char> = Vec::new();
        for char in row.chars() {
            rowv.push(char);
            if char == 'S' {
                start.1 = ycord;
                start.0 = xcord;
            }
            xcord += 1;
        }
        xcord = 0;
        ycord += 1;
        map.push(rowv);
    }

    for row in &map {
        for char in row {
            print!("{}", char);
        }
        println!();
    }
    let mut hashmap : HashMap<(usize, usize), usize> = HashMap::new();
    let mut numsplits = checkElem(&mut map, start.0, start.1, &mut hashmap);
    /*

    let mut total = 0;
    let mut beams : Vec<Vec<(usize, usize)>> = Vec::new();
    let mut startcord :Vec<(usize, usize)> = Vec::new();
    let mut numsplits = 0;
    let mut map2 = map.clone();
    startcord.push((start.0, start.1));
    beams.push(startcord);
    for i in 0..map.len() {
        println!("{}", i);
        let mut newbeams :Vec<(usize, usize)> = Vec::new();
        for beam in &beams[i] {
            if beam.1 < map.len() {
                //println!(" {} {} {}", map[beam.1][beam.0], beam.0, beam.1);
                if map[beam.1][beam.0] == '^' {
                    //if map2[beam.1][beam.0] == '^' {
                        numsplits += 1;
                    //    map2[beam.1][beam.0] = '|'
                    //}
                    if beam.0 > 0 {
                        newbeams.push((beam.0 - 1, beam.1 + 1));
                    }

                    if beam.1 < map[0].len() {
                        newbeams.push((beam.0 + 1, beam.1 + 1));
                    }
                }
                else {
                    newbeams.push((beam.0, beam.1 + 1))
                }
            }
        }
        beams.push(newbeams);
    }
    
    //println!("x: {}, y {}", start.0, start.1);

    for beam in beams {
        for bb in beam {
            //println!("{} {}", bb.0, bb.1);
        }
    }*/

    println!("Total {}", numsplits);
    Ok(())
}