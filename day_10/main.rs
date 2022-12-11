use std::fs::File;
use std::io::{BufRead, BufReader};
use std::iter::FromIterator;


const CYCLES: &[i32] = &[20, 60, 100, 140, 180, 220];

const SCREEN_HEIGTH: usize = 6;
const SCREEN_WIDTH: usize = 40;


struct Screen {
    x: i32,
    cycle: i32,
    signal_strength: i32,
    buffer: [char; SCREEN_HEIGTH*SCREEN_WIDTH],
}

impl Screen {
    fn new() -> Screen {
        Screen {
            x: 1,
            cycle: 0,
            signal_strength: 0,
            buffer: ['~'; SCREEN_HEIGTH*SCREEN_WIDTH],
        }
    }

    fn increment_cycle(&mut self) {
        self.cycle += 1;

        if CYCLES.contains(&self.cycle) {
            self.signal_strength += self.x * self.cycle;
        }


        let index = (self.cycle - 1) as usize % (SCREEN_HEIGTH*SCREEN_WIDTH);
        if (index % SCREEN_WIDTH) as isize >= (self.x-1) as isize
                && (index % SCREEN_WIDTH) as isize <= (self.x+1) as isize {
            self.buffer[index] = '#';
        } else {
            self.buffer[index] = '.';
        }
    }

    fn execute(&mut self, operation: &str) {
        let mut split = operation.split(" ");
        let inst: &str = split.next().expect("Error getting instruction");

        match inst {
            "noop" => self.increment_cycle(),
            "addx" => {
                self.increment_cycle();
                self.increment_cycle();

                let op: &str = split.next().expect("Error getting operand");
                let op: i32 = op.parse::<i32>().expect("Error parsing operand");
                self.x += op;
            },
            default => {
                eprintln!("Invalid instruction '{}'", default);
                std::process::exit(1);
            },
        }
    }

    fn print_buffer(&self) {
        for i in 0..SCREEN_HEIGTH {
            let start = i*SCREEN_WIDTH;
            let end = (i+1)*SCREEN_WIDTH;
            let line: String = String::from_iter(&self.buffer[start..end]);
            println!("{}", line);
        }
    }
}


fn main() {
    let input: File = File::open("input").expect("Error opening input file");
    let reader: BufReader<File> = BufReader::new(input);

    let mut screen: Screen = Screen::new();

    for line in reader.lines() {
        let line: String = line.expect("Error reading file");
        // println!("Instruction: {}", line);
        screen.execute(line.as_str());
        // screen.print_buffer();
        // println!("");
    }

    println!("Part one: {}", screen.signal_strength);
    println!("Part two:");
    screen.print_buffer();
}
