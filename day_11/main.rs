use std::fs::File;
use std::io::{BufRead, BufReader};


#[derive(Clone)]
enum Operation {
    Add,
    Mult,
    Pow,
}

#[derive(Clone)]
struct Monkey {
    items: Vec<u64>,
    operation: Operation,
    operand: Option<u64>,
    test_value: u64,
    test_true: usize,
    test_false: usize,
    inspected_items: u64,
}

impl Monkey {
    fn new(items: Vec<u64>, operation: Operation, operand: Option<u64>,
           test_value: u64, test_true: usize, test_false: usize) -> Monkey {
        Monkey {
            items: items,
            operation: operation,
            operand: operand,
            test_value: test_value,
            test_true: test_true,
            test_false: test_false,
            inspected_items: 0
        }
    }

    fn inspect(&mut self, mcm: u64, divide: bool) {
        for item in self.items.as_mut_slice() {
            match &self.operation {
                Operation::Add => *item += self.operand.unwrap(),
                Operation::Mult => *item *= self.operand.unwrap(),
                Operation::Pow => *item = item.pow(2),
            }
            
            *item %= mcm;

            if divide {
                *item = *item / 3;
            }
            self.inspected_items += 1;
        }

    }

    fn recv_item(&mut self, item: u64) {
        self.items.push(item);
    }

    fn get_monkeys(&self) -> (usize, usize) {
        (self.test_true, self.test_false)
    }

    fn test(&mut self, t_items: &mut Vec<u64>, f_items: &mut Vec<u64>) {
        loop {
            let item = self.items.pop();
            match item {
                None => return,
                Some(item) => {
                    if item % self.test_value == 0 {
                        t_items.push(item);
                    } else {
                        f_items.push(item);
                    }
                },
            }
        }
    }
}


fn parse_monkey(reader: &mut BufReader<File>) -> Monkey {
    let mut line = String::new();
    reader.read_line(&mut line).unwrap();

    let mut split = line.split_whitespace();
    split.next();
    split.next();
    let mut items: Vec<u64> = Vec::new();
    for item in split {
        if item.ends_with(",") {
            items.push(item[0..item.len()-1].parse::<u64>().unwrap());
        } else {
            items.push(item.parse::<u64>().unwrap());
        }
    }

    line.clear();
    reader.read_line(&mut line).unwrap();
    let mut split = line.split_whitespace();
    split.next();
    split.next();
    split.next();
    split.next();
    let op_str: &str = split.next().expect("Error reading op");
    let mut operation: Operation;
    if op_str == "+" {
        operation = Operation::Add;
    } else {
        operation = Operation::Mult;
    }
    let operand: Option<u64> = split.next().unwrap().parse::<u64>().ok();
    if operand.is_none() {
        operation = Operation::Pow
    }

    line.clear();
    reader.read_line(&mut line).unwrap();
    let mut split = line.split_whitespace();
    split.next();
    split.next();
    split.next();
    let test_value: u64 = split.next().unwrap().parse::<u64>().unwrap();

    line.clear();
    reader.read_line(&mut line).unwrap();
    let mut split = line.split_whitespace();
    split.next();
    split.next();
    split.next();
    split.next();
    split.next();
    let test_true: usize = split.next().unwrap().parse::<usize>().unwrap();

    line.clear();
    reader.read_line(&mut line).unwrap();
    let mut split = line.split_whitespace();
    split.next();
    split.next();
    split.next();
    split.next();
    split.next();
    let test_false: usize = split.next().unwrap().parse::<usize>().unwrap();

    Monkey::new(items, operation, operand, test_value, test_true, test_false)
}

fn exec(mut monkeys: Vec<Monkey>, rounds: usize, div: bool, mcm: u64) -> u64 {
    for _ in 0..rounds {
        for i in 0..monkeys.len() {
            let monkey = &mut monkeys[i];
            monkey.inspect(mcm, div);
            let mut vec_true = Vec::<u64>::new();
            let mut vec_false = Vec::<u64>::new();
            monkey.test(&mut vec_true, &mut vec_false);
            let (true_index, false_index) = monkey.get_monkeys();
            for item in vec_true {
                monkeys[true_index].recv_item(item);
            }
            for item in vec_false {
                monkeys[false_index].recv_item(item);
            }
        }
    }

    let mut max: u64 = 0;
    let mut max2: u64 = 0;

    for monkey in monkeys {
        if monkey.inspected_items > max {
            max2 = max;
            max = monkey.inspected_items;
        } else if monkey.inspected_items > max2 {
            max2 = monkey.inspected_items;
        }
    }
    
    max*max2
}

fn main() {
    let input: File = File::open("input").expect("Error opening input file");
    let mut reader: BufReader<File> = BufReader::new(input);

    let mut monkeys: Vec<Monkey> = Vec::new();

    loop {
        let mut line = String::new();
        let readed = reader.read_line(&mut line).unwrap();
        
        if readed == 0 {
            break;
        }

        if line.starts_with("Monkey") {
            monkeys.push(parse_monkey(&mut reader));
        }
    }

    let mut mcm: u64 = 1;
    for monkey in monkeys.as_slice() {
        mcm *= &monkey.test_value;
    }

    println!("Part one: {}", exec(monkeys.clone(), 20, true, mcm));
    println!("Part two: {}", exec(monkeys, 10000, false, mcm));
}
