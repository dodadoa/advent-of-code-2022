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


fn main() -> io::Result<()> {
    let file_path = "./input/3.txt";
    let file = File::open(file_path)?;
    let reader = BufReader::new(file);

    quiz3_part1(reader);

    Ok(())
}
