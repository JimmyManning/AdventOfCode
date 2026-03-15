use std::fs::File;
use std::io::Read;
use std::collections::HashMap;
fn rotatePresent(present : Vec<Vec<char>>) -> Vec<Vec<char>> {
    let mut rotatedPresent : Vec<Vec<char>> = Vec::new();
    let mut row : Vec<char> = Vec::new();
    row.push('.');
    row.push('.');
    row.push('.');
    rotatedPresent.push(row.clone());
    rotatedPresent.push(row.clone());
    rotatedPresent.push(row.clone());

    rotatedPresent[0][2] = present[0][0];
    rotatedPresent[1][2] = present[0][1];
    rotatedPresent[2][2] = present[0][2];
    rotatedPresent[0][1] = present[1][0];
    rotatedPresent[1][1] = present[1][1];
    rotatedPresent[2][1] = present[1][2];
    rotatedPresent[0][0] = present[2][0];
    rotatedPresent[1][0] = present[2][1];
    rotatedPresent[2][0] = present[2][2];

    return rotatedPresent;
}

fn flipHorPresent(present : Vec<Vec<char>>) -> Vec<Vec<char>> {
    let mut rotatedPresent : Vec<Vec<char>> = Vec::new();
    let mut row : Vec<char> = Vec::new();
    row.push('.');
    row.push('.');
    row.push('.');
    rotatedPresent.push(row.clone());
    rotatedPresent.push(row.clone());
    rotatedPresent.push(row.clone());

    rotatedPresent[0][2] = present[0][0];
    rotatedPresent[1][2] = present[0][1];
    rotatedPresent[2][2] = present[0][2];
    rotatedPresent[0][1] = present[1][0];
    rotatedPresent[1][1] = present[1][1];
    rotatedPresent[2][1] = present[1][2];
    rotatedPresent[0][0] = present[2][0];
    rotatedPresent[1][0] = present[2][1];
    rotatedPresent[2][0] = present[2][2];

    return rotatedPresent;
}


fn flipVerPresent(present : Vec<Vec<char>>) -> Vec<Vec<char>> {
    let mut rotatedPresent : Vec<Vec<char>> = Vec::new();
    let mut row : Vec<char> = Vec::new();
    row.push('.');
    row.push('.');
    row.push('.');
    rotatedPresent.push(row.clone());
    rotatedPresent.push(row.clone());
    rotatedPresent.push(row.clone());

    rotatedPresent[0][2] = present[0][0];
    rotatedPresent[1][2] = present[0][1];
    rotatedPresent[2][2] = present[0][2];
    rotatedPresent[0][1] = present[1][0];
    rotatedPresent[1][1] = present[1][1];
    rotatedPresent[2][1] = present[1][2];
    rotatedPresent[0][0] = present[2][0];
    rotatedPresent[1][0] = present[2][1];
    rotatedPresent[2][0] = present[2][2];

    return rotatedPresent;
}


fn fitPresents(grid : Vec<Vec<char>>, presents : Vec<Vec<Vec<char>>>, requiredPresents : Vec<usize>, presentCount : Vec<usize>) -> bool 
{
    return true;
}

fn main() -> std::io::Result<()> {
    let mut file = File::open("input.txt")?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    let mut input = contents.split("\n");
    let mut total = 0;
    let mut currentPresent = 0;
    let mut currentRow = 0;
    let mut presents : Vec<Vec<Vec<char>>> = Vec::new();
    let mut presentGrid : Vec<Vec<char>> = Vec::new();
    let mut gridsize : Vec<(usize, usize)>= Vec::new();
    let mut requiredPresents : Vec<Vec<usize>> = Vec::new();
    for row in input {
        if row.contains("x") {
            let elems : Vec<String> = row.split(":").map(String::from).collect();
            let leftside : Vec<String> = elems[0].split("x").map(String::from).collect();

            let mut usizeXCord: usize = leftside[0].parse::<usize>().expect("Failed to parse string to usize");
            let mut usizeYCord: usize = leftside[1].parse::<usize>().expect("Failed to parse string to usize");
            gridsize.push((usizeXCord, usizeYCord));
            let numpresents : Vec<String> = elems[1].split(" ").map(String::from).collect();
            let mut reqpres : Vec<usize> = Vec::new();
            for presetn in numpresents {
                if presetn.len() > 0 {
                let mut nump: usize = presetn.parse::<usize>().expect("Failed to parse string to usize");
                reqpres.push(nump);
                }
            }
            requiredPresents.push(reqpres);
        }
        else {
            if row.len() == 0 {
                presents.push(presentGrid.clone());
                presentGrid.clear();
            }
            else if row.contains(":") {

            }
            else {
                let mut rowvec : Vec<char> = Vec::new();
                for i in row.chars() {
                    rowvec.push(i);
                }
                presentGrid.push(rowvec);
            }
        }
    }


    for i in 0..presents.len() {
        println!("{}:", i);
        for row in presents[i].clone() {
            for col in row {
                print!("{}", col);
            }
            println!();
        }
        println!();
    }

    for i in 0..requiredPresents.len() {
        print!("{}x{} ", gridsize[i].0, gridsize[i].1);
        for x in 0..requiredPresents[i].len() {
            print!("{} ", requiredPresents[i][x]);
        }
        println!();
    }

    for i in 0..requiredPresents.len() {
        let grid : Vec<Vec<char>> = Vec::new();
        let presentCount : Vec<usize> = Vec::new();
        fitPresents(grid.clone(), presents.clone(), requiredPresents[i].clone(), presentCount);
    }

    for row in presents[0].clone() {
        for col in row {
            print!("{}", col);
        }
        println!();
    }
    println!();
    let mut rotatedPresent = rotatePresent(presents[0].clone());
    for row in rotatedPresent.clone() {
        for col in row {
            print!("{}", col);
        }
        println!();
    }
    println!();
    rotatedPresent = rotatePresent(rotatedPresent.clone());
    for row in rotatedPresent.clone() {
        for col in row {
            print!("{}", col);
        }
        println!();
    }
    println!();
    rotatedPresent = rotatePresent(rotatedPresent.clone());
    for row in rotatedPresent.clone() {
        for col in row {
            print!("{}", col);
        }
        println!();
    }        
    total = 0;
    for i in 0..requiredPresents.len() {
        let mut area = gridsize[i].0 * gridsize[i].1;
        let mut reqarea = 0;
        for reqpres in requiredPresents[i].clone() {
            reqarea += (9 * reqpres);
        }
        println!("{} {}", area, reqarea);
        if area >= reqarea {
            total += 1;
        }
    }

    println!("Total {}", total);
    Ok(())
}