use std::fs::File;
use std::io::{BufRead, BufReader};


struct Operation {
    cranes: u32,
    from: usize,
    to: usize,
}


fn parse_initial_state(reader: &mut BufReader<File>) -> Vec<Vec<char>> {
    let mut init_lines: Vec<String> = Vec::with_capacity(32);

    loop {
        let mut line: String = String::new();
        let _ = reader.read_line(&mut line).expect("Error reading file");
        line.pop(); // Remove final new line char
        if line.len() == 0 {
            break;
        }
        init_lines.push(line);
    }

    let line: String = init_lines.pop().expect("Unable to pop");
    let n_stacks = line[line.len()-2..line.len()-1].parse::<usize>().unwrap();
    let mut stacks: Vec<Vec<char>> = Vec::with_capacity(n_stacks);
    for _ in 0..n_stacks {
        stacks.push(Vec::new());
    }

    loop {
        match init_lines.pop() {
            Some(line) => {
                for i in 0..n_stacks {
                    let c: char = line.as_bytes()[i*4+1] as char;
                    if c.is_ascii_alphabetic() {
                        stacks[i].push(c);
                    }
                }
            },
            None => break,
        }
    }

    stacks
}

fn get_opretaion(line: String, operation: &mut Operation) {
    let mut n = 0;
    let mut result: [u32; 3] = [0; 3];
    for slice in line.split_whitespace() {
        if slice.chars().next().unwrap().is_ascii_digit() {
            result[n] = slice.parse::<u32>().expect("Unable to parse");
            n += 1;
        }
    }
    operation.cranes = result[0];
    operation.from = result[1] as usize -1;
    operation.to = result[2] as usize -1;
}

fn get_top_cranes(stacks: Vec<Vec<char>>) -> String {
    let mut top_cranes: String = String::new();
    for stack in stacks {
        top_cranes.push(*stack.last().expect("No cranes in this stack"));
    }
    top_cranes
}

fn main() {
    let input: File = File::open("input").expect("Error opening input file");
    let mut reader: BufReader<File> = BufReader::new(input);

    let mut stacks_1: Vec<Vec<char>> = parse_initial_state(&mut reader);
    let mut stacks_2 = stacks_1.clone();

    for line in reader.lines() {
        let line: String = line.expect("Error reading file");
        let mut op: Operation = Operation{cranes: 0, from: 0, to: 0};
        get_opretaion(line, &mut op);

        // Part one
        for _ in 0..op.cranes {
            let crane = stacks_1[op.from].pop().expect("Unable to pop");
            stacks_1[op.to].push(crane);
        }

        // Part two
        let index = stacks_2[op.from].len() - op.cranes as usize;
        let mut aux = stacks_2[op.from].drain(index..).collect::<Vec<_>>();
        stacks_2[op.to].append(&mut aux);
    }

    let top_cranes_1 = get_top_cranes(stacks_1);
    let top_cranes_2 = get_top_cranes(stacks_2);
    println!("Part one: {}", top_cranes_1);
    println!("Part two: {}", top_cranes_2);
}
