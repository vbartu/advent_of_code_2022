use std::fs::File;
use std::io::{BufRead, BufReader};


struct Section {
    min: u32,
    max: u32,
}


fn parse_input(line: String)-> (Section, Section) {
    let mut parser: String = String::with_capacity(8);
    let mut values: [u32; 4] = [0; 4];
    
    let mut i: usize = 0;
    for c in line.chars() {
        if c.is_ascii_digit() {
            parser.push(c);
            values[i] = parser.parse::<u32>().expect("Unable to parse string");
        } if c == '-' || c == ',' {
            i += 1;
            parser.clear();
        }
    }
    
    let s1: Section = Section {
        min: values[0],
        max: values[1],
    };
    let s2: Section = Section {
        min: values[2],
        max: values[3],
    };

    (s1, s2)
}

fn main() {
    let input: File = File::open("input").expect("Error opening input file");
    let reader: BufReader<File> = BufReader::new(input);

    let mut n_contained: u32 = 0;
    let mut n_overlap: u32 = 0;

    for line in reader.lines() {
        let line: String = line.expect("Error reading file");
        let (s1, s2): (Section, Section) = parse_input(line);

        // Part one
        if (s1.min <= s2.min && s1.max >= s2.max)
                || (s1.min >= s2.min && s1.max <= s2.max) {
            n_contained += 1;
        }

        // Part two
        if (s1.max >= s2.min && s1.min <= s2.max)
                || (s2.max >= s1.min && s2.min <= s1.max) {
            n_overlap += 1;
        }

    }

    println!("Part one: {}", n_contained);
    println!("Part two: {}", n_overlap);
}
