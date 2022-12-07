use std::fs::File;
use std::io::{BufRead, BufReader};


fn main() {
    let input = File::open("input").expect("Error opening input file");
    let reader = BufReader::new(input);

    let mut calories: u32 = 0;
    let mut max_calories: [u32; 3] = [0; 3];

    for line in reader.lines() {
        let line = line.expect("Error reading file");
        if line.len() == 0 {
            if calories > max_calories[2] {
                max_calories[2] = calories;
            }

            if calories > max_calories[1] {
                max_calories[2] = max_calories[1];
                max_calories[1] = calories;
            }

            if calories > max_calories[0] {
                max_calories[1] = max_calories[0];
                max_calories[0] = calories;
            }
            calories = 0;
        } else {
            calories += line.parse::<u32>().unwrap();
        }
    }

    println!("Part one: {}", max_calories[0]);
    println!("Part two: {}", max_calories.iter().sum::<u32>());
}
