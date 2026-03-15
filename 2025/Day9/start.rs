use std::fs::File;
use std::io::Read;
use std::collections::HashMap;
fn getDistance(p1 : usize, p2 : usize) -> f64 {
        let squared : f64 = ((p1 as f64 - p2 as f64).powf(2.0));
        let distance = squared.sqrt();
    return distance;
}


fn main() -> std::io::Result<()> {
    let mut file = File::open("input.txt")?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    let mut x = contents.split("\n");
    let mut total = 0;
    let mut redTiles : Vec<(usize, usize)> = Vec::new();
    let mut grid : Vec<Vec<char>> = Vec::new();
    let mut maxX = 0;
    let mut maxY = 0;
    let mut minX = 65535;
    let mut minY = 65535;
    let mut xCoords : Vec<usize> = Vec::new();
    let mut xyCoords : Vec<usize> = Vec::new();
    for row in x {
        let (xcord, ycord) = row.split_once(',')
                                     .expect("String should contain a dash");
        //println!("{} - {}", low, high);

        let mut usizeXCord: usize = xcord.parse::<usize>().expect("Failed to parse string to usize");
        let mut usizeYCord: usize = ycord.parse::<usize>().expect("Failed to parse string to usize");

        redTiles.push((usizeXCord, usizeYCord));
        if usizeYCord < minY {
            minY = usizeYCord
        }
        if usizeXCord < minX {
            minX = usizeXCord
        }        
        if usizeXCord > maxX {
            maxX = usizeXCord;
        }
        if usizeYCord > maxY {
            maxY = usizeYCord;
        }
        xyCoords.push(usizeXCord);
        xyCoords.push(usizeYCord);
    }
    let mut maxArea = 0.0;
    // for i in 0..redTiles.len() {
    //     for j in i..redTiles.len() {
    //         let mut width = getDistance(redTiles[i].0, redTiles[j].0) + 1.0;
    //         let mut length = getDistance(redTiles[i].1, redTiles[j].1) + 1.0;
    //         println!("{}", width * length);
    //         if width * length > maxArea {
    //             maxArea = width * length;
    //         }
    //     }
    // }
    println!("Make the grid {} {} {} {}", maxX, maxY, minX, minY);
    maxY = maxY - minY;
    maxX = maxX - minX;

    xCoords.sort();
    xyCoords.sort();
    let mut compressedMap : HashMap<usize, usize> = HashMap::new();
    let mut uncompressedMap : HashMap<usize, usize> = HashMap::new();
    for i in 0..xyCoords.len() {
        compressedMap.insert(i, xyCoords[i]);
        uncompressedMap.insert(xyCoords[i], i);
    }

    let mut compressedRedTiles : Vec<(usize, usize)> = Vec::new();
    maxX = 0;
    maxY = 0;
    for i in 0..redTiles.len() {
        let mut compressedX : usize = uncompressedMap[&redTiles[i].0];
        let mut compressedY : usize = uncompressedMap[&redTiles[i].1];
        if compressedX > maxX {
            maxX = compressedX;
        }
        if compressedY > maxY {
            maxY = compressedY;
        }        
        compressedRedTiles.push((compressedX, compressedY));

    }

    for y in 0..maxY+1 {
        let mut row : Vec<char> = Vec::new();
        for x in 0..maxX+1 {
            row.push('.');
        }
        grid.push(row);
    }
    let mut lastx = 0;
    let mut lasty = 0;
    let mut drop = false;
    for mut i in 0..compressedRedTiles.len()+1 {
        if i == compressedRedTiles.len() {
            i = i - 1;
            lastx = compressedRedTiles[0].0;
            lasty = compressedRedTiles[0].1;
            drop = true;
        }

        if i == 0 {
            println!("{} {} {} {}", lastx, lasty, compressedRedTiles[i].0, compressedRedTiles[i].1);
            grid[compressedRedTiles[i].1][compressedRedTiles[i].0] = 'X'
        }
        else {
            println!("{} {} {} {}", lastx, lasty, compressedRedTiles[i].0, compressedRedTiles[i].1);
            if compressedRedTiles[i].0 == lastx {
                if lasty < compressedRedTiles[i].1 {
                    for j in lasty..compressedRedTiles[i].1 {
                        grid[j][lastx] = 'X';
                    }
                }
                else 
                {
                    for j in compressedRedTiles[i].1..lasty {
                        grid[j][lastx] = 'X';
                    }
                }
            }
            else if compressedRedTiles[i].1 == lasty {
                if lastx < compressedRedTiles[i].0 {
                    for j in lastx..compressedRedTiles[i].0 {
                        grid[lasty][j] = 'X';
                    }
                }
                else 
                {
                    for j in compressedRedTiles[i].0..lastx {
                        grid[lasty][j] = 'X';
                    }
                }
            }
        }
        lastx = compressedRedTiles[i].0;
        lasty = compressedRedTiles[i].1;
        if drop {
            break;
        }
    }
    let mut unfilledGrid = grid.clone();
    for y in 0..grid.len() {
        for x in 0..grid[0].len() {
            if unfilledGrid[y][x] != 'X' {
                let mut intersects = 0;
                let mut lastval = '.';
                let mut strings = 0;
                let mut trynext = false;
                for i in (0..y).rev()
                {
                    if unfilledGrid[i][x] == 'X' {
                        strings = strings + 1;
                    }
                    else {
                        if lastval == 'X' {
                            if strings == 1 {
                                intersects = intersects + 1;
                            }
                            else {
                                trynext = true;
                            }
                        }
                        strings = 0;
                    }
                    lastval = unfilledGrid[i][x];
                }
                if trynext {
                    intersects = 0;
                    lastval = '.';
                    strings = 0;
                    trynext = false;
                    for i in (0..x).rev()
                    {
                        if unfilledGrid[y][i] == 'X' {
                            strings = strings + 1;
                        }
                        else {
                            if lastval == 'X' {
                                if strings == 1 {
                                    intersects = intersects + 1;
                                }
                                else {
                                    trynext = true;
                                }
                            }
                            strings = 0;
                        }
                        lastval = unfilledGrid[y][i];
                    }
                }
                if trynext {
                    intersects = 0;
                    lastval = '.';
                    strings = 0;
                    trynext = false;
                    for i in (x..grid[0].len()).rev()
                    {
                        if unfilledGrid[y][i] == 'X' {
                            strings = strings + 1;
                        }
                        else {
                            if lastval == 'X' {
                                if strings == 1 {
                                    intersects = intersects + 1;
                                }
                                else {
                                    trynext = true;
                                }
                            }
                            strings = 0;
                        }
                        lastval = unfilledGrid[y][i];
                    }
                }                
                if intersects % 2 == 1 {
                    //if strings == 1 || strings < intersects {
                        grid[y][x] = 'X';
                    //}
                    // else {

                    //             let mut intersects = 0;
                    //             let mut lastval = '.';
                    //             let mut strings = 0;
                    //             for i in x..grid.len() {
                    //                 if unfilledGrid[y][i] == 'X' {
                    //                     strings = strings + 1;
                    //                 }
                    //                 if unfilledGrid[y][i] == 'X' && lastval != 'X' {
                    //                     intersects = intersects + 1;
                    //                 }
                    //                 lastval = unfilledGrid[y][i];
                    //             }
                    //             if intersects % 2 == 1 {
                    //                 if strings == 1 || strings > 1 && intersects > 1 {
                    //                     grid[y][x] = 'X';
                    //                 }
                    //             }
                    // } 
                }
            }
        }    
    }

    maxArea = 0.0;
    for i in 0..compressedRedTiles.len() {
        for j in i..compressedRedTiles.len() {
            let mut width = getDistance(compressedMap[&compressedRedTiles[i].0], compressedMap[&compressedRedTiles[j].0]) + 1.0;
            let mut length = getDistance(compressedMap[&compressedRedTiles[i].1], compressedMap[&compressedRedTiles[j].1]) + 1.0;
            //println!("{}", width * length);
            let mut skip = false;
            let mut startX = 0;
            let mut endX = 0;
            let mut startY = 0;
            let mut endY = 0;
            // if (compressedRedTiles[i].1..compressedRedTiles[j].1).len() == (compressedMap[&compressedRedTiles[i].1]..compressedMap[&compressedRedTiles[j].1]).len() &&
            //     (compressedRedTiles[i].0..compressedRedTiles[j].0).len() == (compressedMap[&compressedRedTiles[i].0]..compressedMap[&compressedRedTiles[j].0]).len() {
            //     skip = false;
            // }
            if compressedRedTiles[i].1 > compressedRedTiles[j].1 {
                endY = compressedRedTiles[i].1;
                startY = compressedRedTiles[j].1;
            }
            else 
            {
                endY = compressedRedTiles[j].1;
                startY = compressedRedTiles[i].1;
            }
            

            if compressedRedTiles[i].0 > compressedRedTiles[j].0 {
                endX = compressedRedTiles[i].0;
                startX = compressedRedTiles[j].0;
            }
            else 
            {
                endX = compressedRedTiles[j].0;
                startX = compressedRedTiles[i].0;
            }
            
            for y in startY..endY {
                if !skip {
                    for x in startX..endX {
                        if grid[y][x] != 'X' {
                            skip = true;
                           // println!();
                          //  println!();
                            break;
                        }
                        //print!("{}", 'X');
                    }
                    //println!();
                }
            }
            if !skip && width * length > maxArea {
                
                maxArea = width * length;
                println!("found a max {} {} {} {} {}", maxArea, compressedRedTiles[i].0, compressedRedTiles[i].1, compressedRedTiles[j].0, compressedRedTiles[j].0);
            }

        }
    }

    for y in 0..grid.len() {
        for x in 0..grid[0].len() {
            print!("{}", grid[y][x]);
        }
        println!();
    }
    

    println!("Total {}", maxArea);
    Ok(())
}