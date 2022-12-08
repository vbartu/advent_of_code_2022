use std::fs::File;
use std::io::{Read,Seek,BufReader};


fn check_not_equal(buffer: &[u8]) -> bool {
    for i in 0..buffer.len() {
        for j in i+1..buffer.len() {
            if buffer[i] == buffer[j] {
                return false;
            }
        }
    }
    true
}

fn get_chars_until_not_equal(reader: &mut BufReader<File>, size: usize) -> u32 {
    let mut buffer: Vec<u8> = vec![0; size];
    let n_read = reader.read(&mut buffer[..size-1]).expect("Error reading");

    let mut n_chars: u32 = n_read as u32;
    let mut n: usize = n_read;
    loop {
        let n_read = reader.read(&mut buffer[n..n+1]).expect("Error reading");
        n_chars += 1;
        n = (n+1) % buffer.len();

        if check_not_equal(buffer.as_slice()) || n_read == 0 {
            break;
        }
    }
    n_chars
}

fn main() {
    let input: File = File::open("input").expect("Error opening input file");
    let mut reader: BufReader<File> = BufReader::new(input);

    let n_chars_pkt: u32 = get_chars_until_not_equal(&mut reader, 4);
    reader.rewind().expect("Error rewinding file");
    let n_chars_msg: u32 = get_chars_until_not_equal(&mut reader, 14);

    println!("Part one: {}", n_chars_pkt);
    println!("Part two: {}", n_chars_msg);
}
