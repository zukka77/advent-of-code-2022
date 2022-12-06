use std::collections::HashSet;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    let mut lines = read_lines("./INPUT").expect("error reading file");
    if let Ok(answer1) = first_question(lines) {
        println!("{}", answer1);
    };
    lines = read_lines("./INPUT").expect("error reading file");
    if let Ok(answer2) = second_question(lines) {
        println!("{}", answer2);
    };
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn first_question(lines: io::Lines<io::BufReader<File>>) -> io::Result<usize> {
    let input_lines: Vec<String> = lines.map(|line| line.unwrap()).collect();
    let input_data = &input_lines[0].as_bytes();
    for i in 0..input_data.len() - 4 {
        let mut char_set: HashSet<u8> = HashSet::new();
        char_set.extend(&input_data[i..i + 4]);
        if char_set.len() == 4 {
            return Ok(i + 4);
        }
    }
    panic!("packet marker not found");
}

fn second_question(lines: io::Lines<io::BufReader<File>>) -> io::Result<usize> {
    let input_lines: Vec<String> = lines.map(|line| line.unwrap()).collect();
    let input_data = &input_lines[0].as_bytes();
    for i in 0..input_data.len() - 14 {
        let mut char_set: HashSet<u8> = HashSet::new();
        char_set.extend(&input_data[i..i + 14]);
        if char_set.len() == 14 {
            return Ok(i + 14);
        }
    }

    panic!("message marker not found");
}
