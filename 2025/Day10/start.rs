use std::fs::File;
use std::io::Read;
use std::collections::HashMap;
use std::collections::HashSet;
use std::convert::TryInto;

fn findOptions(currentState : Vec<char>, desieredState : Vec<char>, buttons : Vec<Vec<usize>>) -> Vec<Vec<usize>>
{
    let mut options : Vec<Vec<usize>> = Vec::new();
    let max = (2_i32).pow(buttons.len().try_into().unwrap());
    //println!("{} {}", max, buttons.len());
    for i in 0..max {
        let mut pushedbutton : Vec<usize> = Vec::new();
        let mut triedState = currentState.clone();
        for j in 0..buttons.len() {
            if (i >> j) & 0x1 == 1 {
                //println!("pushing button {}", j);
                pushedbutton.push(1);
                pushButton(&mut triedState, buttons.clone(), j);
            }
            else {
                pushedbutton.push(0);
            }
        }
        //println!("-{:?}", pushedbutton); 

        let mut equal = true;
        //println!("Does {:?} == {:?}", triedState, desieredState);
        for j in 0..triedState.len() {
            if triedState[j] != desieredState[j] {
                equal = false;
            }
        }
        if equal {
            options.push(pushedbutton);
        }
    }
    return options;
}

fn findShortest2(buttons : Vec<Vec<usize>>, currentJoltages : Vec<usize>, deep : usize) -> usize
{
    let mut shortest : usize = 0;
    let mut initialState : Vec<char> = Vec::new();
    let mut desiered : Vec<char> = Vec::new();
    let mut somepushed : bool = false;
    let mut alldone : bool = true;
    for x in 0..currentJoltages.len() {
        initialState.push('.');
        if currentJoltages[x] % 2 == 1 {
            desiered.push('#');
            somepushed = true;
        }
        else {
            desiered.push('.');
        }

        if currentJoltages[x] != 0 {
            alldone = false;
        }
    }
    if alldone {
        return 0 ;
    }
    let mut pushedButtons : Vec<Vec<usize>> = Vec::new();

    // for i in 0..deep {
    //     print!(" ");
    // }
    //println!("going for {:?} {:?}", desiered, currentJoltages);
    //if somepushed {

        pushedButtons = findOptions(initialState, desiered.clone(), buttons.clone());
    //}

    let mut pushes = 0;
    let mut myJoltages = currentJoltages.clone();

    // if !somepushed {
    //     let mut all0 : bool = true;
    //     for i in 0..myJoltages.len() 
    //     {
    //         if (myJoltages[i] == 1)
    //         {
    //             return 65535;
    //         }
    //         if myJoltages[i] != 0 {
    //             all0 = false;
    //         }
    //         myJoltages[i] = myJoltages[i] / 2;
    //     }
    //     if !all0 {
    //         let mut shortest = findShortest2(buttons.clone(), myJoltages.clone(), deep + 1);
    //         if (shortest < 65535) {
    //             pushes += shortest * 2;
    //         }
    //         else {
    //             return 65535;
    //         }
    //     }
    // }
    //else {
        let mut shortpath = 65535;
        
        for pushedButton in pushedButtons.clone() {
//println!("trying {:?}", pushedButton);
            let mut nextJoltages = currentJoltages.clone();
            let mut failed : bool = false;
            let mut toadd = 0;
            let mut all0 : bool = true;
            let mut alleven : bool = true;
            for i in 0..pushedButton.len() {
                if pushedButton[i] == 1 {
                    if decrementcounts(&mut nextJoltages, buttons.clone(), i) == false {
                        //println!("NEGTAIVE FAILURE");
                        failed = true;
                        break;
                    }
                    else {
                        toadd += 1;
                    }
                }
            }
            for nextJoltage in &nextJoltages {
                if *nextJoltage != 0 {
                    all0 = false;
                    break;
                }
                if *nextJoltage % 2 != 0 {
                    alleven = false;
                    break;
                }
            }

            //if all0 {
            //    return toadd;
            //}
            if alleven {
                for i in 0..nextJoltages.len() {
                    nextJoltages[i] /= 2;
                }
            }
            
            if !failed {
                let mut currentPath = findShortest2(buttons.clone(), nextJoltages.clone(), deep + 1);
                if alleven {
                    currentPath *= 2;
                }
                if (currentPath + toadd) < shortpath {
                    shortpath = currentPath + toadd;
                }
            }
        }
        pushes = shortpath;
    //}

    return pushes;
}

fn incrementcounts(currentState : &mut Vec<usize>, buttons : Vec<Vec<usize>>, buttonToPush : usize) {
    for i in 0..buttons[buttonToPush].len()
    {
        currentState[buttons[buttonToPush][i]] = currentState[buttons[buttonToPush][i]] + 1;
    }

}


fn decrementcounts(currentState : &mut Vec<usize>, buttons : Vec<Vec<usize>>, buttonToPush : usize) -> bool 
{
    for i in 0..buttons[buttonToPush].len()
    {
        if currentState[buttons[buttonToPush][i]] == 0 {
            return false;
        }
        currentState[buttons[buttonToPush][i]] = currentState[buttons[buttonToPush][i]] - 1;
    }
    return true;
}

fn findShortestjoltage(currentState : Vec<usize>, desieredState : Vec<usize>, buttons : Vec<Vec<usize>>, deep : usize, currentShortest : usize, maxdepth : usize, seen : &mut HashMap<String, usize> ) -> usize
{
    let mut shortest = currentShortest;
    // for i in &currentState {
    //     print!("{}", i);
    // }
    // print!("  ");
    // for i in &desieredState {
    //     print!("{}", i);
    // }
    // println!("{}", currentShortest);
    // if currentShortest < deep || deep > maxdepth {
    //     return 65535;
    // }
    let mut equal : bool = true;
    for i in 0..currentState.len() {
        if currentState[i] != desieredState[i] {
            if currentState[i] > desieredState[i] {
                return 65525;
            }
            equal = false;
        }
    }
    if equal {
        //println!("WE FOUND AN EQUAL");
        return 0;
    }
    else 
    {
        let s: String = currentState
        .iter()
        .map(|&num| num.to_string())
        .collect::<Vec<String>>()
        .join(", "); // You can choose any separator

        if seen.contains_key(&s) {
            if *seen.get(&s).unwrap() <= deep {
                //println!("found {} again", s);
                return 65535;
            }
            else {
                 seen.insert(s.clone(), deep);
            }
        }
        else {
                 seen.insert(s.clone(), deep);   
        }

        for i in 0..buttons.len() {
            let mut nextState = currentState.clone();
            incrementcounts(&mut nextState, buttons.clone(), i);
            let mut val = findShortestjoltage(nextState, desieredState.clone(), buttons.clone(), deep + 1, shortest, maxdepth, seen) + 1;
            if val < shortest {
                shortest = val;
            }
        }
    }
    return shortest;
}
fn pushButton(currentState : &mut Vec<char>, buttons : Vec<Vec<usize>>, buttonToPush : usize) {
    for i in 0..buttons[buttonToPush].len() {
        if currentState[buttons[buttonToPush][i]] == '#' {
            currentState[buttons[buttonToPush][i]]= '.';
        }
        else {
            currentState[buttons[buttonToPush][i]]= '#';
        }
    }

}

fn findShortest(currentState : Vec<char>, desieredState : Vec<char>, buttons : Vec<Vec<usize>>, deep : usize, currentShortest : usize, maxdepth : usize, seen : &mut HashMap<String, usize>, currentJoltages : Vec<usize>, pushedButtons : Vec<usize>) -> (usize, Vec<usize>, HashSet<Vec<usize>>)
{
    let mut shortestJoltages = currentJoltages.clone();
    let mut shortedPushedButtons : HashSet<Vec<usize>> = HashSet::new();
    let mut shortest = currentShortest;
    // for i in &currentState {
    //     print!("{}", i);
    // }
    // print!("  ");
    // for i in &desieredState {
    //     print!("{}", i);
    // }
    // println!("{}", currentShortest);
    // if deep > maxdepth {
    //     shortedPushedButtons.clear();
    //     return (65535, shortestJoltages, shortedPushedButtons);
    // }
    let mut equal : bool = true;
    for i in 0..currentState.len() {
        if currentState[i] != desieredState[i] {
            equal = false;
        }
    }
    if equal {
        //println!("WE FOUND AN EQUAL");
        shortedPushedButtons.insert(pushedButtons.clone());
        return (0, shortestJoltages, shortedPushedButtons);
    }
    else 
    {
        // let s: String = currentState.clone().into_iter().collect();
        // if seen.contains_key(&s) {
        //     if *seen.get(&s).unwrap() <= deep {
        //         shortedPushedButtons.clear();
        //         return (65535, shortestJoltages, shortedPushedButtons);
        //     }
        //     else {
        //          seen.insert(s.clone(), deep);
        //     }
        // }
        // else {
        //          seen.insert(s.clone(), deep);   
        // }
        let mut anypushed : bool = false;
        for i in 0..buttons.len() {
            let mut nextState = currentState.clone();
            let mut newJoltages = currentJoltages.clone();
            let mut nextPushedButtons = pushedButtons.clone();
            if nextPushedButtons[i] == 0 {
                anypushed = true;
                nextPushedButtons[i] += 1;
                pushButton(&mut nextState, buttons.clone(), i);
                incrementcounts(&mut newJoltages, buttons.clone(), i);
                let (mut val, mut joltages, mut pushedButtons) = findShortest(nextState, desieredState.clone(), buttons.clone(), deep + 1, shortest, maxdepth, seen, newJoltages.clone(), nextPushedButtons.clone());
                val += 1;
                if val < 65545 {
                    for pushedButton in pushedButtons {
                        if !shortedPushedButtons.contains(&pushedButton) {
                            shortedPushedButtons.insert(pushedButton);
                        }
                    }
                }
                if val < shortest {
                    shortest = val;
                    shortestJoltages = joltages;
                    //shortedPushedButtons = pushedButtons;
                }
            }
        }
        if !anypushed {
            return (shortest, shortestJoltages, shortedPushedButtons);
        }
    }
    return (shortest, shortestJoltages, shortedPushedButtons);
}

struct Machine {
    lights : Vec<char>,
    buttons : Vec<Vec<usize>>,
    joltages : Vec<usize>,
}

fn main() -> std::io::Result<()> {
    let mut file = File::open("input.txt")?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    let mut input = contents.split("\n");
    let mut total = 0;

    let mut lights : Vec<Vec<char>> = Vec::new();
    let mut buttons : Vec<Vec<Vec<usize>>> = Vec::new();
    let mut joltages : Vec<Vec<usize>> = Vec::new();
    let mut machines : Vec<Machine> = Vec::new();
    for row in input {
        let mut elems :Vec<_> = row.split(" ").collect();
        let mut light : Vec<char> = Vec::new();
        for i in 1..elems[0].len()-1 {
            light.push(elems[0].chars().nth(i).unwrap());
        }
        lights.push(light.clone());
        let mut machineButtons : Vec<Vec<usize>> = Vec::new();
        for i in 1..elems.len()-1 {
            let mut ops : Vec<usize> = Vec::new();
            let splitoptions : Vec<_> = elems[i][1..elems[i].len()-1].split(",").collect();
            for splitop in splitoptions {
                let uval32: usize = splitop.parse::<usize>().expect("Failed to parse string to u64");
                ops.push(uval32);
            }
            machineButtons.push(ops);
        }
        buttons.push(machineButtons.clone());
        let mut joltage : Vec<usize> = Vec::new();
        let splitjoltages : Vec<_> = elems[elems.len() - 1][1..elems[elems.len() - 1].len()-1].split(",").collect();
        for splitjoltage in splitjoltages {
            let uval32: usize = splitjoltage.parse::<usize>().expect("Failed to parse string to u64");
            joltage.push(uval32);
        }
        joltages.push(joltage.clone());
        machines.push(Machine{lights: light, buttons: machineButtons, joltages: joltage});

    }

    for button in &buttons {
        for elem in button {
            print!("(");
            for print in elem {
                print!("{},", print);
            }
            print!("), ");
        }
    }
    println!();


    let mut part2total = 0;
    for i in 0..lights.len() {
        let mut curval = findShortest2(buttons[i].clone(), joltages[i].clone(), 0);
        if (curval >= 65535)
        {
            println!("BBBBBADDDD");
        }
        println!("{} {}", i, curval);
        part2total = part2total + curval;
    }

    // for i in 0..lights.len() {
    //     let mut initialState : Vec<usize> = Vec::new();
    //     for x in 0..lights[i].len() {
    //     //    print!("{}", lights[i][x]);
    //         initialState.push(0)
    //     }
    //     let mut shortest = 65535;
    //     // for j in 0..10 {
    //     //     println!("depth {}", j);
    //         let mut hashset : HashMap<String, usize> = HashMap::new();
    //         shortest = findShortestjoltage(initialState.clone(), joltages[i].clone(), buttons[i].clone(), 0, 65535, 1000, &mut hashset);
    //     //     if shortest < 65535 {
    //     //         break;
    //     //     }
    //     // } 
    //     println!("shortest was {} for {}", shortest, i);
    //     part2total = part2total + shortest;
    // }    
    

    println!("Total {} {}", total, part2total);
    Ok(())
}