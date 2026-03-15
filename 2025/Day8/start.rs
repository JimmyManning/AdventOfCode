use std::fs::File;
use std::io::Read;
use std::collections::HashMap;
use std::collections::HashSet;


fn numConnected(connectedBoxes : &mut Vec<HashSet<usize>>, index : usize, tried : &mut Vec<usize>) -> usize
{
    let mut connected = 0;
    
    for connect in &connectedBoxes[index].clone() {
        if !tried.contains(&connect) {
            tried.push(*connect);
            connected += numConnected(connectedBoxes, *connect, tried) + 1;
        }
    }
    return connected;
}

fn isConnected(junctionBoxes: &mut Vec<(usize, usize, usize)>, connectedBoxes : &mut Vec<HashSet<usize>>, one : usize, two : usize, tried : &mut Vec<usize>) -> bool {
    let mut connected : bool = false;
    if connectedBoxes[one].contains(&two) {
        connected = true;
    }

    if connected == false {
        tried.push(one);
        for connect in connectedBoxes[one].clone() {
            if !tried.contains(&connect) {
                if isConnected(junctionBoxes, connectedBoxes, connect, two, tried) {
                    connected = true;
                    break;
                }
            }
        }
    }
    return connected;
}

fn getDistance(x1 : usize, y1 : usize, z1 : usize, x2 : usize, y2 : usize, z2 : usize) -> f64 {
        let squared : f64 = ((x1 as f64 - x2 as f64).powf(2.0) + (y1 as f64 - y2 as f64).powf(2.0) + (z1 as f64 - z2 as f64).powf(2.0)) as f64;
        let distance = squared.sqrt();
    return distance;
}

fn findClosest(junctionBoxes: &mut Vec<(usize, usize, usize)>, connectedBoxes : &mut Vec<HashSet<usize>>, distancesMap : &mut HashMap<(usize, usize), f64>) -> (usize, usize) {
    let mut closestDistance : f64 = -1.0;
    let mut closestIndex : (usize, usize) = (0, 0);
    let mut distancesCalculated = 0;
    for i in 0..junctionBoxes.len() {
        for j in i+1..junctionBoxes.len() {
                let mut tried : Vec<usize> = Vec::new();
                if !isConnected(junctionBoxes, connectedBoxes, i, j, &mut tried) {
                    let founddistance = distancesMap.get(&(i, j));
                    let mut distance : f64 = 0.0;
                    match founddistance {
                        Some(score) => distance = *founddistance.unwrap(),
                        None => distance = getDistance(junctionBoxes[i].0, junctionBoxes[i].1, junctionBoxes[i].2, junctionBoxes[j].0, junctionBoxes[j].1, junctionBoxes[j].2),
                    }
                    distancesMap.insert((i, j), distance);
                    // distance = getDistance(junctionBoxes[i].0, junctionBoxes[i].1, junctionBoxes[i].2, junctionBoxes[j].0, junctionBoxes[j].1, junctionBoxes[j].2);
                    if distancesCalculated == 0 || distance < closestDistance {
                        closestDistance = distance;
                        closestIndex = (i, j);
                    }
                    distancesCalculated += 1;
                }
                else {
                    connectedBoxes[i].insert(j);
                    connectedBoxes[j].insert(i);
                }
        }
    }

    return closestIndex;
}

fn main() -> std::io::Result<()> {
    let mut file = File::open("input.txt")?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    let mut x = contents.split("\n");
    let mut total = 0;

    let mut junctionBoxes : Vec<(usize, usize, usize)> = Vec::new();
    let mut connectedBoxes : Vec<HashSet<usize>> = Vec::new();
    for row in x {
        let mut elems = row.split(",");
        let mut vals : Vec<usize> = Vec::new();
        for elemt in elems {
            let mut uvalusize: usize = elemt.parse::<usize>().expect("Failed to parse string to usize");
            vals.push(uvalusize);
        }

        junctionBoxes.push((vals[0], vals[1], vals[2]));
        let mut connections : HashSet<usize> = HashSet::new();
        connectedBoxes.push(connections);
    }
    let mut NumberOfChecked = 0;
    let mut ClosestDistance = 0.0;
    let mut ClosestIndexs : (usize, usize) = (0, 0);

    for i in 0..junctionBoxes.len() {
        println!("{} {} {}", junctionBoxes[i].0, junctionBoxes[i].1, junctionBoxes[i].2)
    }
    let mut allconnections : Vec<Vec<usize>>;
    let mut distancesMap : HashMap<(usize, usize), f64> = HashMap::new();
    let mut val = findClosest(&mut junctionBoxes.clone(), &mut connectedBoxes, &mut distancesMap);
    let mut sorted_items_distance: Vec<(&(usize, usize), &f64)> = distancesMap.iter().collect();
    sorted_items_distance.sort_by(|a, b| a.1.total_cmp(b.1));
    for i in &sorted_items_distance {
        println!("{} {} {}", i.0.0, i.0.1, i.1);
    }
    let mut last_sorted = 0;
    for i in 0..999 {
        for j in last_sorted..sorted_items_distance.len() {
            let mut tried : Vec<usize> = Vec::new();
            if !isConnected(&mut junctionBoxes, &mut connectedBoxes, sorted_items_distance[j].0.0, sorted_items_distance[j].0.1, &mut tried) {
            //if !connectedBoxes[sorted_items_distance[j].0.0].contains(&sorted_items_distance[j].0.1) {
                connectedBoxes[sorted_items_distance[j].0.0].insert(sorted_items_distance[j].0.1);
                connectedBoxes[sorted_items_distance[j].0.1].insert(sorted_items_distance[j].0.0);
                println!( "Connecting {} -> {} {} {}", sorted_items_distance[j].0.0, sorted_items_distance[j].0.1, connectedBoxes[sorted_items_distance[j].0.0].len(), connectedBoxes[sorted_items_distance[j].0.1].len());
                println!( " {} {} {} ", junctionBoxes[sorted_items_distance[j].0.0].0, junctionBoxes[sorted_items_distance[j].0.1].0, junctionBoxes[sorted_items_distance[j].0.0].0 *junctionBoxes[sorted_items_distance[j].0.1].0);
                last_sorted = j;
                break;
            }
            else {
                //last_sorted = j;
                //connectedBoxes[sorted_items_distance[j].0.0].insert(sorted_items_distance[j].0.1);
                //connectedBoxes[sorted_items_distance[j].0.1].insert(sorted_items_distance[j].0.0);
                //break;
            }
        }
        //val = findClosest(&mut junctionBoxes.clone(), &mut connectedBoxes, &mut distancesMap);
        println!("{}", i);
        //println!("{} {} {} {} {} {} {}", i, junctionBoxes[val.0].0, junctionBoxes[val.0].1, junctionBoxes[val.0].2, junctionBoxes[val.1].0, junctionBoxes[val.1].1, junctionBoxes[val.1].2);
    }
    let mut map : HashMap<usize, usize> = HashMap::new();
    for i in 0..junctionBoxes.len() {
        let mut tried : Vec<usize> = Vec::new();
        tried.push(i);
        let mut num = numConnected(&mut connectedBoxes, i, &mut tried);
        println!("{} {}", num, tried.len());
        map.insert(tried.len(), i);
    }
    let mut sorted_items: Vec<(&usize, &usize)> = map.iter().collect();
    sorted_items.sort_by_key(|(key, value)| **key);
    for elem in sorted_items {
        println!("--{} {}", elem.0, elem.1);
    }

    println!("Total {}", total);
    Ok(())
}