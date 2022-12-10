use std::fs::File;
use std::io::{BufRead, BufReader};
use std::collections::HashSet;


#[derive(Eq,PartialEq,Hash,Clone)]
struct Point {
    x: i32,
    y: i32,
}

impl Point {
    fn sub(&self, other: &Point) -> Point {
        Point {x: self.x - other.x, y: self.y - other.y}
    }

    fn distance(&self, other: &Point) -> f32 {
        let diff = self.sub(other);
        ((diff.x.pow(2) + diff.y.pow(2)) as f32).sqrt()
    }
}


struct Rope {
    knots: Vec<Point>,
    tail_positions: HashSet<Point>,
}

impl Rope {
    fn new(size: usize) -> Self {
        let start_point = Point {x: 0, y: 0};
        let vec: Vec<Point> = vec![start_point.clone(); size];
        let mut set = HashSet::new();
        set.insert(start_point.clone());
        Self {knots: vec, tail_positions: set}
    }

    fn execute(&mut self, op: char, reps: u32) {
        match op {
            'U' => for _ in 0..reps { self.move_up() },
            'D' => for _ in 0..reps { self.move_down() },
            'R' => for _ in 0..reps { self.move_right() },
            'L' => for _ in 0..reps { self.move_left() },
             default => eprintln!("Invalid op {}", default),
        }
    }

    fn move_up(&mut self) {
        self.knots[0].y += 1;
        self.update_tail();
    }

    fn move_down(&mut self) {
        self.knots[0].y -= 1;
        self.update_tail();
    }

    fn move_right(&mut self) {
        self.knots[0].x += 1;
        self.update_tail();
    }

    fn move_left(&mut self) {
        self.knots[0].x -= 1;
        self.update_tail();
    }

    fn update_tail(&mut self) {
        for i in 1..self.knots.len() {

            if self.knots[i-1].distance(&self.knots[i]) < 2.0 {
                return
            }

            let diff = self.knots[i-1].sub(&self.knots[i]);
            if diff.x != 0 {
                self.knots[i].x += diff.x / diff.x.abs();
            }
            if diff.y != 0 {
                self.knots[i].y += diff.y / diff.y.abs();
            }

        }

        self.tail_positions.insert(self.knots[self.knots.len()-1].clone());
    }

    fn get_tail_positions(&self) -> usize {
        self.tail_positions.len()
    }
}


fn main() {
    let input: File = File::open("input").expect("Error opening input file");
    let reader: BufReader<File> = BufReader::new(input);

    let mut rope_1 = Rope::new(2);
    let mut rope_2 = Rope::new(10);

    for line in reader.lines() {
        let line: String = line.expect("Error reading file");

        let mut split = line.split(" ");
        let op: char = split.next().unwrap().chars().next().unwrap();
        let reps: u32 = split.next().unwrap().parse::<u32>().unwrap();

        rope_1.execute(op, reps);
        rope_2.execute(op, reps);
    }

    println!("Part one: {}", rope_1.get_tail_positions());
    println!("Part two: {}", rope_2.get_tail_positions());
}
