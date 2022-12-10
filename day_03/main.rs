use std::fs::File;
use std::io::{BufRead, BufReader};


fn get_priority(c: char) -> u32 {
    if c.is_ascii_lowercase() {
        (c as u8 - 'a' as u8) as u32 + 1
    } else { // uppercase
        (c as u8 - 'A' as u8) as u32 + 27
    }
}

fn main() {
    let input = File::open("input").expect("Error opening input file");
    let reader = BufReader::new(input);

    let mut priority_1: u32 = 0;
    let mut priority_2: u32 = 0;
    let mut elf_group: [String; 3] = Default::default();

    for (i, line) in reader.lines().enumerate() {
        let line = line.expect("Error reading file");
        let (half_1, half_2): &(&str, &str) = &line.split_at(line.len()/2);

        // Part one
        for c in half_1.chars() {
            if half_2.contains(c) {
                priority_1 += get_priority(c);
                break;
            }
        }

        // Part two
        elf_group[i%3] = line;
        if i%3 == 2 {
            for c in elf_group[0].chars() {
                if elf_group[1].contains(c) && elf_group[2].contains(c) {
                    priority_2 += get_priority(c);
                    break;
                }
            }
        }
    }

    println!("Part one: {}", priority_1);
    println!("Part two: {}", priority_2);
}
