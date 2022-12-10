use std::fs::File;
use std::io::BufReader;
use std::io::BufRead;


const DIR_THRESHOLD: u32 = 100000;
const DISK_SPACE: u32 = 70000000;
const UPDATE_SIZE: u32 = 30000000;


fn calculate_size(vec: &mut Vec<u32>, reader: &mut BufReader<File>) -> u32 {
    let mut size: u32 = 0;

    loop {
        let mut line = String::new();
        let readed = reader.read_line(&mut line).expect("Error read_line");
        let line = line.trim(); // Remove new line char

        if readed == 0 {
            break;
        }

        if line.as_bytes()[0].is_ascii_digit() {
            size += &line.split(' ').next().unwrap().parse().unwrap();
        }
        else if &line[0..3] == "dir" {
            // Ignore
        }
        else if &line[0..4] == "$ ls" {
            // Ignore
        }
        else if line.len() >= 7 && &line[0..7] == "$ cd .." {
            break;
        }
        else if &line[0..4] == "$ cd" {
            size += calculate_size(vec, reader);
        }
    }

    vec.push(size);
    size
}


fn main() {
    let input = File::open("input").expect("Error opening input file");
    let mut reader = BufReader::new(input);

    let mut vec: Vec<u32> = Vec::new();

    let mut line = String::new();
    reader.read_line(&mut line).expect("Error read_line");
    let line = line.trim(); // Remove new line char
    assert!(line == "$ cd /", "Input must start with '$ cd /'");

    let total_size = calculate_size(&mut vec, &mut reader);
    println!("Total size: {}", total_size);

    // Part one

    let mut n = 0;
    for v in &vec {
        if v <= &DIR_THRESHOLD {
            n += v;
        }
    }

    println!("Part one result: {}", n);


    // Part two

    let space_left = DISK_SPACE - total_size;
    println!("Space left: {}", space_left);
    let space_needed = UPDATE_SIZE - space_left;
    println!("Space needed: {}", space_needed);


    let mut closest = total_size;
    for v in &vec {
        if v >= &space_needed {
            closest = std::cmp::min(closest, *v);
        }
    }

    println!("Part two result: {}", closest);
}
