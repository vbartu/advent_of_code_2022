use std::fs::File;
use std::io::{BufRead, BufReader};


fn check_visible(map: &Vec<Vec<u8>>, x: usize, y: usize) -> bool {
    if x == 0 || y == 0 || x == map.len()-1 || y == map[0].len()-1 {
        return true;
    }
    
    let value: u8 = map[x][y];

    // Check Up
    let mut check: bool = true;
    for i in 0..x {
        if map[i][y] >= value {
            check = false;
            break;
        }
    }

    if check {
        return true;
    }

    // Check Down
    check = true;
    for i in x+1..map.len() {
        if map[i][y] >= value {
            check = false;
            break;
        }
    }

    if check {
        return true;
    }

    // Check Right
    check = true;
    for i in 0..y {
        if map[x][i] >= value {
            check = false;
            break;
        }
    }

    if check {
        return true;
    }

    // Check Left
    check = true;
    for i in y+1..map[0].len() {
        if map[x][i] >= value {
            check = false;
            break;
        }
    }

    check
}

fn get_score(map: &Vec<Vec<u8>>, x: usize, y: usize) -> u32 {
    let value: u8 = map[x][y];

    // Check Up
    let mut up: u32 = 0;
    if x > 0 {
        for i in (0..x).rev() {
            if map[i][y] < value {
                up += 1;
            }
            if map[i][y] >= value {
                up += 1;
                break;
            }
        }
    }

    // Check Down
    let mut down: u32 = 0;
    if x < map.len()-1 {
        for i in x+1..map.len() {
            if map[i][y] < value {
                down += 1;
            }
            if map[i][y] >= value {
                down += 1;
                break;
            }
        }
    }

    // Check Right
    let mut right: u32 = 0;
    if y > 0 {
        for i in (0..y).rev() {
            if map[x][i] < value {
                right += 1;
            }
            if map[x][i] >= value {
                right += 1;
                break;
            }
        }
    }

    // Check Left
    let mut left: u32 = 0;
    if y < map[0].len()-1 {
        for i in y+1..map[0].len() {
            if map[x][i] < value {
                left += 1;
            }
            if map[x][i] >= value {
                left += 1;
                break;
            }
        }
    }

    up * down * right * left
}




fn main() {
    let input: File = File::open("input").expect("Error opening input file");
    let reader: BufReader<File> = BufReader::new(input);

    let mut map: Vec<Vec<u8>> = Vec::new();

    for line in reader.lines() {
        let line = line.expect("Error reading file");
        let mut v: Vec<u8> = Vec::new();

        for c in line.chars() {
            v.push(c as u8 - '0' as u8);
        }
        map.push(v);
    }

    let mut visible: u32 = 0;
    let mut max_score: u32 = 0;

    for i in 0..map.len() {
        for j in 0..map[0].len() {
            // Part one
            if check_visible(&map, i, j) {
                visible += 1;
            }
            // Part two
            max_score = std::cmp::max(max_score, get_score(&map, i, j));
        }
    }

    println!("Part one: {}", visible);
    println!("Part two: {}", max_score);
}
