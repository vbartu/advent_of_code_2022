use std::fs::File;
use std::io::{BufRead, BufReader};


/**
 * Part one:
 * A,X Rock
 * B,Y Paper
 * C,Z Scissors
 *
 * Part two:
 * X Lose
 * Y Draw
 * Z Win
 */


fn calculate_score_1(opponent: &str, you: &str) -> u32 {
    let mut score: u32 = 0;

    if you == "X" {
        score += 1;
        if opponent == "C" { // Win
            score += 6
        } else if opponent == "A" { // Draw
            score += 3;
        }
    }
    else if you == "Y" {
        score += 2;
        if opponent == "A" { // Win
            score += 6
        } else if opponent == "B" { // Draw
            score += 3;
        }
    }
    else { // you == "Z"
        score += 3;
        if opponent == "B" { // Win
            score += 6
        } else if opponent == "C" { // Draw
            score += 3;
        }
    }
    
    score
}

fn calculate_score_2(opponent: &str, result: &str) -> u32 {
    let mut score: u32 = 0;
    
    if result == "X" {
        if opponent == "A" { // Rock
            score += 3
        } else if opponent == "B" { // Paper
            score += 1
        } else { // opponent == "C" // Scissors
            score += 2
        }
    }
    else if result == "Y" {
        score += 3;
        if opponent == "A" { // Rock
            score += 1
        } else if opponent == "B" { // Paper
            score += 2
        } else { // opponent == "C" // Scissors
            score += 3
        }
    } else { // result == "Z"
        score += 6;
        if opponent == "A" { // Rock
            score += 2
        } else if opponent == "B" { // Paper
            score += 3
        } else { // opponent == "C" // Scissors
            score += 1
        }
    }

    score
}

fn main() {
    let input: File = File::open("input").expect("Error opening input file");
    let reader: BufReader<File> = BufReader::new(input);

    let mut score_1: u32 = 0;
    let mut score_2: u32 = 0;

    for line in reader.lines() {
        let line = line.expect("Unable to read file");
        let (v1, v2) = line.split_once(" ").expect("Unable to split line");
        score_1 += calculate_score_1(v1, v2);
        score_2 += calculate_score_2(v1, v2);
    }

    println!("Part one: {}", score_1);
    println!("Part two: {}", score_2);
}
