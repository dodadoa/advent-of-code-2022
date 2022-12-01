use std::collections::BinaryHeap;
use std::fs::File;
use std::io::{self, prelude::*, BufReader};

fn part1(reader: BufReader<File>) -> io::Result<()>  {

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

fn part2(reader: BufReader<File>) -> io::Result<()>  {
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


fn main() -> io::Result<()> {
    let file_path = "./input/1.txt";
    let file = File::open(file_path)?;
    let reader = BufReader::new(file);

    part2(reader);

    Ok(())
}
