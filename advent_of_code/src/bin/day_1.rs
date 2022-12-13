use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let filename = "resources/2022-day-1";
    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);

    let mut localSum = 0; // will hold sum for each elf
    let mut max1 = 0;
    let mut max2 = 0;
    let mut max3 = 0;

    // Read the file line by line using the lines() iterator from std::io::BufRead.
    for (index, line) in reader.lines().enumerate() {
        let line = line.unwrap(); // Ignore errors.
        if line == "" {
            if localSum >= max1 {
                max3 = max2;
                max2 = max1;
                max1 = localSum
            } else if localSum >= max2  {
                max3 = max2;
                max2 = localSum;
            } else if localSum >= max3  {
                max3 = localSum;
            }
            localSum = 0;
        } else {
            let line_int = line.parse::<i32>().unwrap();
            localSum += line_int;
        }
    }
    println!("Max: {}", max1);
    println!("Collective Top-3 Max: {}", max1 + max2 + max3);
}