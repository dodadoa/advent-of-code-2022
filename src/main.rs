use std::collections::{BinaryHeap, HashSet};
use std::fs::File;
use std::io::{self, prelude::*, BufReader};

// quiz 1

fn quiz1_part1(reader: BufReader<File>) -> io::Result<()>  {

    let mut max: u32 = 0;
    let mut temp: u32 = 0;

    for line in reader.lines() {
        let value = line.unwrap();
        if value.eq("") {
            if temp > max {
                max = temp;
            }
           temp = 0;
        } else {
            let number_value: u32 = value.parse::<u32>().unwrap();
            temp = temp + number_value;
        }
    }

    println!("{}", max);

    Ok(())
}

fn quiz1_part2(reader: BufReader<File>) -> io::Result<()>  {
    let mut top_three = BinaryHeap::new();
    let mut temp: u32 = 0;

    for line in reader.lines() {
        let value = line.unwrap();
        if value.eq("") {
            top_three.push(temp);
           
            temp = 0;
        } else {
            let number_value: u32 = value.parse::<u32>().unwrap();
            temp = temp + number_value;
        }
    }

    let top_1 = top_three.pop().unwrap();
    let top_2 = top_three.pop().unwrap();
    let top_3 = top_three.pop().unwrap();

    println!("{}", top_1 + top_2+ top_3);

    Ok(())
}

// quiz 2

fn quiz2_part1 (reader: BufReader<File>) -> io::Result<()> {

    let mut total_score: u32 = 0;
    for line in reader.lines() {
        let l = line.unwrap();
        let e = l.split(" ").collect::<Vec<&str>>();
        match e[..] {
            [a, "X"] => {
                total_score = total_score + 1;
                if a.eq("A") {
                    total_score = total_score + 3;
                }
                if a.eq("B") {
                    total_score = total_score + 0;
                }
                if a.eq("C") {
                    total_score = total_score + 6;
                }
            },
            [a, "Y"] => {
                // Y is paper
                total_score = total_score + 2;
                
                if a.eq("A") {
                    total_score = total_score + 6;
                }
                if a.eq("B") {
                    total_score = total_score + 3;
                }
                if a.eq("C") {
                    total_score = total_score + 0;
                }
            },
            [a, "Z"] => {
                total_score = total_score + 3;
                if a.eq("A") {
                    total_score = total_score + 0;
                }
                if a.eq("B") {
                    total_score = total_score + 6;
                }
                if a.eq("C") {
                    total_score = total_score + 3;
                }
            },
            [_, _] => println!("error"),
            [_] => println!("error"),
            [_, _, _, ..] => println!("error"),
            [] => println!("error")
        }
    }

    println!("{}", total_score);

    Ok(())
}

fn quiz2_part2 (reader: BufReader<File>) -> io::Result<()> {

    // X means you need to lose, 
    // Y means you need to end the round in a draw, 
    // and Z means you need to win. Good luck!"

    let mut total_score: u32 = 0;
    for line in reader.lines() {
        let l = line.unwrap();
        let e = l.split(" ").collect::<Vec<&str>>();
        match e[..] {
            // to lose
            [a, "X"] => {
                // rock
                // choose scissors -> 3 + 0
                if a.eq("A") {
                    total_score = total_score + 3;
                }
                // paper
                // chose rock -> 1 + 0
                if a.eq("B") {
                    total_score = total_score + 1;
                }
                // scissor
                // choose paper -> 2 + 0
                if a.eq("C") {
                    total_score = total_score + 2;
                }
            },
            // to draw
            [a, "Y"] => {

                // to draw -> + 3
                total_score = total_score + 3;
                // rock
                // choose rock ->+ 1
                if a.eq("A") {
                    total_score = total_score + 1;
                }
                // paper
                // choose paper -> + 2
                if a.eq("B") {
                    total_score = total_score + 2;
                }
                // scissors
                // choose scissors -> + 3
                if a.eq("C") {
                    total_score = total_score + 3;
                }
            },
            [a, "Z"] => {
                // to win -> + 6
                total_score = total_score + 6;
                // rock
                // choose paper -> + 2
                if a.eq("A") {
                    total_score = total_score + 2;
                }
                // paper
                // choose scissors -> + 3
                if a.eq("B") {
                    total_score = total_score + 3;
                }
                // scissors
                // choose rock -> + 1
                if a.eq("C") {
                    total_score = total_score + 1;
                }
            },
            [_, _] => println!("error"),
            [_] => println!("error"),
            [_, _, _, ..] => println!("error"),
            [] => println!("error")
        }
    }

    println!("{}", total_score);

    Ok(())
}


fn quiz3_part1(reader: BufReader<File>) -> io::Result<()> {
    let alphabet: Vec<&str> = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ".split("").collect();

    let mut dup: Vec<String> = vec![];
    for line in reader.lines() {
        let rucksack = line.unwrap();
        let rucksack_size = rucksack.len();
        let compartment_size = rucksack_size / 2;
        let first_compartment = rucksack[0..compartment_size].to_owned();
        let second_compartment = rucksack[compartment_size..rucksack_size].to_owned();

        let first: HashSet<&str> = first_compartment.split("").into_iter().collect();
        let second: HashSet<&str> = second_compartment.split("").into_iter().collect();

        println!("{first:?} {second:?}");

        for first_item in first {
            if first_item.eq("") {
                continue;
            }
            if second.contains(&first_item) {
                dup.push(first_item.to_owned());
            }
        }
    }

    let mut sum: usize = 0;
    for al in dup {
        let found = alphabet.iter().find(|a| a.eq(&&al)).unwrap();
        let found_index = alphabet.iter().position(|a| a.eq(&&al)).unwrap();
        println!("{found} {found_index}");
        sum = sum + found_index;
    }

    println!("{sum}");

    Ok(())
}

fn quiz3_part2(reader: BufReader<File>) -> io::Result<()> {
    let alphabet: Vec<&str> = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ".split("").collect();

    let mut item_badge: Vec<String> = vec![];
    let mut whole: Vec<Vec<String>> = vec![];
    let mut group: Vec<String> = vec![];
    for line in reader.lines() {
        let rucksack = line.unwrap();
        group.push(rucksack);
        if group.len() == 3 {
            whole.push(group);
            group = vec![];
        }
    }

    for each_group in whole {
        let first: HashSet<&str> = each_group[0].split("").into_iter().collect();
        let second: HashSet<&str> = each_group[1].split("").into_iter().collect();
        let third: HashSet<&str> = each_group[2].split("").into_iter().collect();

        for first_item in first {
            if first_item.eq("") {
                continue;
            }
            if second.contains(&first_item) {
                if third.contains(&first_item) {
                    item_badge.push(first_item.to_owned());
                }
            }
        }
    }


    let mut sum: usize = 0;
    for al in item_badge {
        let found = alphabet.iter().find(|a| a.eq(&&al)).unwrap();
        let found_index = alphabet.iter().position(|a| a.eq(&&al)).unwrap();
        println!("{found} {found_index}");
        sum = sum + found_index;
    }

    println!("{sum}");

    Ok(())
}

fn quiz4_part1(reader: BufReader<File>) -> io::Result<()> {
    let mut total_cover_whole = 0;
    for line in reader.lines() {
        let section = line.unwrap();
        let pairs: Vec<&str> = section.split(",").into_iter().collect();
        let first_pair: Vec<&str> = pairs[0].split("-").into_iter().collect();
        let second_pair: Vec<&str> = pairs[1].split("-").into_iter().collect();

        let first_pair_begin = first_pair[0].parse::<i32>().unwrap();
        let first_pair_end = first_pair[1].parse::<i32>().unwrap();

        let second_pair_begin = second_pair[0].parse::<i32>().unwrap();
        let second_pair_end = second_pair[1].parse::<i32>().unwrap();

        // .2345678.  2-8
        // ..34567..  3-7
        if first_pair_begin <= second_pair_begin && first_pair_end >= second_pair_end {
            total_cover_whole = total_cover_whole + 1;
            continue;
        }

        // .....6...  6-6
        // ...456...  4-6
        if second_pair_begin <= first_pair_begin && second_pair_end >= first_pair_end {
            total_cover_whole = total_cover_whole + 1;
            continue;
        }
    }

    println!("{total_cover_whole}");

    Ok(())
}

fn quiz4_part2(reader: BufReader<File>) -> io::Result<()> {
    let mut total_cover_whole = 0;
    for line in reader.lines() {
        let section = line.unwrap();
        let pairs: Vec<&str> = section.split(",").into_iter().collect();
        let first_pair: Vec<&str> = pairs[0].split("-").into_iter().collect();
        let second_pair: Vec<&str> = pairs[1].split("-").into_iter().collect();

        let first_pair_begin = first_pair[0].parse::<i32>().unwrap();
        let first_pair_end = first_pair[1].parse::<i32>().unwrap();

        let second_pair_begin = second_pair[0].parse::<i32>().unwrap();
        let second_pair_end = second_pair[1].parse::<i32>().unwrap();

        println!("{first_pair:?} {second_pair:?}");

        //  case 1
        //   2-4
        //       6-8
    
        // ....567..  5-7
        // .....6789  6-9
        if first_pair_begin <= second_pair_begin && first_pair_end >= second_pair_begin {
            total_cover_whole = total_cover_whole + 1;
            continue;
        }

        // ......789  7-9
        // ....567..  5-7
        if second_pair_begin <= first_pair_begin && second_pair_end >= first_pair_begin { 
            total_cover_whole = total_cover_whole + 1;
            continue;
        }
    }

    println!("{total_cover_whole}");

    Ok(())
}

fn quiz5_part1(reader: BufReader<File>) -> io::Result<()> {
    

    Ok(())
}

fn main() -> io::Result<()> {
    let file_path = "./input/4.txt";
    let file = File::open(file_path)?;
    let reader = BufReader::new(file);

    quiz5_part1(reader);

    Ok(())
}
