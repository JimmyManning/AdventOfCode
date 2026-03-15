use std::fs::File;
use std::io::Read;
use std::collections::HashMap;
use std::collections::HashSet;

fn findGoodPaths(map : HashMap<String, Vec<String>>, paths : &mut HashSet<String>, newPaths : Vec<HashSet<String>>, hi : &mut Vec<Vec<HashSet<String>>>)
{
    hi.push(newPaths.clone());
    let mut myNewPaths : Vec<HashSet<String>> = Vec::new();
    for newpathouter in newPaths {
        for newPath in newpathouter {
            let mut myNewPath : HashSet<String> = HashSet::new();
            for (key, value) in &map {
                if value.contains(&newPath) {
                    myNewPath.insert(key.to_string());
                    paths.insert(key.to_string());
                }
            }
            myNewPaths.push(myNewPath);
        }
    }
    if myNewPaths.len() != 0 {
        findGoodPaths(map, paths, myNewPaths, hi);
    }
}

// fn findPaths(map : HashMap<String, Vec<String>>, paths : &mut HashSet<String>, currentPath : String, currentStep : String, depth : usize, been : HashSet<String>, dacPaths : HashSet<String>, fftPaths : HashSet<String>, outPaths : HashSet<String>, dachi : Vec<Vec<HashSet<String>>>, ffthi : Vec<Vec<HashSet<String>>>) -> usize {
//     let mut numPaths :usize = 0;
//     static mut i : i32 = 0;
//     unsafe {
//     if { let tmp = i; i += 1; tmp } == 10000 {
//         println!("num=------------------ {}", currentPath);
//         i = 0;
//     }
//     }
//     //println!("{} {}", currentStep, currentPath);
//     let mut mybeen = been.clone();
//     //if depth > 10  {
//     //      return 0;
//     //}
//     if been.contains(&currentStep) {
//         return 0;
//     }
//     else {
//         mybeen.insert(currentStep.clone());
//     }
//     if currentStep == "out" {
//         //println!("and out");
//         if currentPath.contains("dac") && currentPath.contains("fft") {
//             if !paths.contains(&currentPath) {
//                 paths.insert(currentPath.clone());
//                 println!("found a path {}", currentPath);
//                 numPaths += 1;
//             }
//         }
//     }
//     else {
//         //println!("{} {}", currentStep, map.contains_key(&currentStep));
//         for path in map.get(&currentStep).unwrap() {
//             //println!("check {}", path);
//             let mut skip = false;
//             if path.len() > 0 {
//                 if currentPath.contains("dac") || currentPath.contains("fft") {
//                     //println!("has one");
//                 }

//                 if currentPath.contains("dac") && currentPath.contains("fft") {
//                     println!("has both");
//                 }
//                 else if currentPath.contains("dac") && !fftPaths.contains(path) {
//                     continue;
//                 } 
//                 //else if currentPath.contains("fft") && !dacPaths.contains(path) {
//                 //    continue;
//                 //}
//                 else {
//                     if !currentPath.contains("fft") && path != "fft" {
//                         for fft in &ffthi[0..ffthi.len()-1] {
//                             let mut pathexhausted : bool = false;
//                             let mut oktogo : bool = false;
//                             for elem in fft {
//                                 if elem == path {
//                                     oktogo = true;
//                                     break;
//                                 }
//                                 if currentPath.contains(elem) {
//                                     pathexhausted = true;
//                                 }
//                             }
//                             if !oktogo && pathexhausted {
//                                 //println!("Path Exhausted {} {}", path, currentPath);
//                                 //for elem in fft {
//                                 //    print!("{} ", elem);
//                                 //}
//                                 //println!();
//                                 skip = true;
//                                 break;
//                             }   
//                             if oktogo {
//                                 break;
//                             }
//                         }
//                     }
//                     if !skip {
//                         if !currentPath.contains("dac") && path != "dac" {
//                             for fft in &dachi[0..dachi.len()-1] {
//                                 let mut pathexhausted : bool = false;
//                                 let mut oktogo : bool = false;
//                                 for elem in fft {
//                                     if elem == path {
//                                         oktogo = true;
//                                         break;
//                                     }
//                                     if currentPath.contains(elem) {
//                                         pathexhausted = true;
//                                     }
//                                 }
//                                 if !oktogo && pathexhausted {
//                                     //println!("Path Exhausted {} {}", path, currentPath);
//                                     //for elem in fft {
//                                     //    print!("{} ", elem);
//                                     //}
//                                     //println!();
//                                     skip = true;
//                                     break;
//                                 }   
//                                 if oktogo {
//                                     break;
//                                 }
//                             }
//                         }
//                     }
//                 }
//                 if !skip {
//                 //println!("jump into{} {}", path, currentPath);
//                 numPaths += findPaths(map.clone(), paths, currentPath.clone() + &currentStep, path.to_string(), depth + 1, mybeen.clone(), dacPaths.clone(), fftPaths.clone(), outPaths.clone(), dachi.clone(), ffthi.clone());
//                 }
//                 skip = false;
//             }
//         }
//     }
//     return numPaths;
// }

fn dfs(
    device: String,
    fft: bool,
    dac: bool,
    graph: HashMap<String, Vec<String>>,
    memo: &mut HashMap<(String, bool, bool), usize>,
) -> usize {
    if device == "out" {
        return if fft && dac { 1 } else { 0 };
    }

    let mut paths_count = 0;

    let next_fft = fft || device == "fft";
    let next_dac: bool = dac || device == "dac";
println!("{} {}", device, graph.contains_key(&device));
    for next_device in graph.get(&device).unwrap() {
        if next_device.len() != 0 {
            if let Some(m) = memo.get(&(next_device.to_string(), next_fft, next_dac)) {
                paths_count += m;
            } else {
                let r = dfs(next_device.to_string(), next_fft, next_dac, graph.clone(), memo);
                paths_count += r;
                memo.insert((next_device.clone(), next_fft, next_dac), r);
            }
        }
    }

    paths_count
}


fn main() -> std::io::Result<()> {
    let mut file = File::open("input.txt")?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    let mut input = contents.split("\n");
    let mut total = 0;

    let mut map : HashMap<String, Vec<String>> = HashMap::new();
    for row in input {
        println!("{}", row);
        let mut step = row.split(":").map(String::from);
        let mut elems : Vec<_> = step.collect();
        println!("{}", elems[1]);
        let mut outs : Vec<_> = elems[1].split(" ").map(String::from).collect();
        map.insert(elems[0].clone(), outs);
    }
    
    let mut newPaths1 : HashSet<String> = HashSet::new();
    let mut newPathVec : Vec<HashSet<String>> = Vec::new();
    let mut dacPaths : HashSet<String> = HashSet::new();
    let mut fftPaths : HashSet<String> = HashSet::new();
    let mut outPaths : HashSet<String> = HashSet::new();

    let mut dachi : Vec<Vec<HashSet<String>>> = Vec::new();
    let mut ffthi : Vec<Vec<HashSet<String>>> = Vec::new();
    let mut outhi : Vec<HashSet<String>> = Vec::new();

    newPaths1.insert("dac".to_string());
    newPathVec.push(newPaths1.clone());
    //findGoodPaths(map.clone(), &mut dacPaths, newPathVec.clone(), &mut dachi);
    println!("DAC PATHS {}", dacPaths.len());
    // for d in &dachi {
    //     for elem in d {
    //         print!("{} ", elem);
    //     }
    //     println!();
    // }

    newPaths1.clear();
    newPaths1.insert("fft".to_string());
    newPathVec.clear();
    newPathVec.push(newPaths1.clone());
    //findGoodPaths(map.clone(), &mut fftPaths, newPathVec.clone(), &mut ffthi);
    println!("FFT PATHS {}", fftPaths.len());
    // newPaths1.clear();
    // newPaths1.insert("out".to_string());
    // let mut goodPaths2 : HashSet<String> = HashSet::new();
    // findGoodPaths(map.clone(), &mut outPaths, newPaths1, &mut outhi);
    // println!("OUT PATHS {}", outPaths.len());

    let mut currentStep : String = "svr".to_string();
    let mut currentPath : String = String::new();
    let mut paths : HashSet<String> = HashSet::new();
    let mut been : HashSet<String> = HashSet::new();
    //total = findPaths(map, &mut paths, currentPath, currentStep, 0, been, dacPaths, fftPaths, outPaths, dachi, ffthi);

    let mut memo : HashMap<(String, bool, bool), usize> = HashMap::new();
    total = dfs(currentStep, false, false, map.clone(), &mut memo);


    println!("Total {}", total);
    Ok(())
}