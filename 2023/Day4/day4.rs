use std::fs::File;
use std::io::Read;
use std::collections::HashSet;

fn main() -> std::io::Result<()> {
    let mut file = File::open("puzzle.txt")?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    println!("{}", contents);

    let mut my_list: Vec<String> = vec![];
    let mut x = contents.split('\n');
    let mut index = 0;
    loop {
        let y = x.next();
        match y{
            Some(y) => {
                index = index + 1;
                my_list.push(y.to_string());
            },
            None => {
                break;
            }
        }
    }
    let mut cards_win_nums: Vec<HashSet<u32>> = vec![];
    let mut cards_my_nums: Vec<HashSet<u32>> = vec![];
    let mut all_card_val: u32 = 0;
    for card in &my_list {
       let parts = card.split(" ");
       let mut winning: bool = true;
       let mut win_nums: HashSet<u32> = HashSet::new();
       let mut my_nums: HashSet<u32> = HashSet::new();
       for part in parts {
            match part {
                x if x.contains("|") => {
                    winning = false;
                },
                x if x.contains("Card") => {

                },
                x if x.contains(":") => {

                },
                x if x.len() == 0 => {

                }
                _ => {
                    if winning {
                        match part.parse::<u32>() {
                            Ok(x) => {
                                win_nums.insert(x);
                            }
                            Err(..) => {
                                println!("Found Parse error");
                            }
                        }
                    } else {
                        match part.parse::<u32>() {
                            Ok(x) => {
                                my_nums.insert(x);
                            }
                            Err(..) => {
                                println!("Found Parse error");
                            }
                        }
                    }
                }
            }
        }
        let mut card_value: u32 = 0;
        for num in &my_nums {
            if win_nums.contains(&num) {
                println!(" found {}", num);
                if card_value == 0 {
                    card_value = 1;
                } else {
                    card_value *= 2;
                }
            }
        }
        println!("Card {}", card_value);
        all_card_val += card_value;

        cards_win_nums.push(win_nums);
        cards_my_nums.push(my_nums);
    }
    let mut cards:Vec<usize> = (0..my_list.len()).collect();
    let mut total_cards: usize = my_list.len();
    //cards.push(1);

    while let Some(top) = cards.pop() {
        let mut wins: usize = 0;
        for my_num in &cards_my_nums[top as usize] {
            if cards_win_nums[top as usize].contains(&my_num) {
                wins += 1;
            }
        }
        for i in 0..wins {
            cards.push(top + i + 1);
            total_cards += 1;
        }
    }


    println!("All cards {}", all_card_val);
    println!("Total Cards {}", total_cards);
    Ok(())
}