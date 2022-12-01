use std::fs::File;
use std::io::{self, prelude::*, BufReader};

fn main() -> io::Result<()> {
    let file_path = "./input/1.txt";
    let file = File::open(file_path)?;
    let reader = BufReader::new(file);

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