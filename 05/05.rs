use lazy_static::lazy_static;
use regex::Regex;
use std::collections::HashMap;
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

fn move_crates(instruction: &str, mut stacks: HashMap<u32, Vec<char>>) -> HashMap<u32, Vec<char>> {
    lazy_static! {
        static ref MOVES_RE: Regex =
            Regex::new(r"move (?P<N>\d+) from (?P<FROM>\d+) to (?P<TO>\d+)").unwrap();
    }
    let captures = MOVES_RE.captures(instruction).unwrap();
    let n = captures.name("N").unwrap().as_str().parse::<u32>().unwrap();
    let from_stack_index = captures
        .name("FROM")
        .unwrap()
        .as_str()
        .parse::<u32>()
        .unwrap();
    let to_stack_index = captures
        .name("TO")
        .unwrap()
        .as_str()
        .parse::<u32>()
        .unwrap();
    for _ in 0..n {
        let mut crate_to_move;
        //BORROWLAND o,O
        {
            let from_stack = stacks.get(&from_stack_index).unwrap();
            crate_to_move = from_stack[from_stack.len() - 1..].to_vec();
        }
        {
            let to_stack: &mut Vec<_> = stacks.get_mut(&to_stack_index).unwrap();
            to_stack.append(&mut crate_to_move);
        }
        stacks.get_mut(&from_stack_index).unwrap().pop();
    }
    stacks
}

fn first_question(lines: io::Lines<io::BufReader<File>>) -> io::Result<String> {
    let input_lines: Vec<String> = lines.map(|line| line.unwrap()).collect();
    let mut stacks: HashMap<u32, Vec<char>> = HashMap::new();
    let mut index: usize = 0;
    //parse crates config
    for i in 0..input_lines.len() {
        let line = &input_lines[i];
        index += 1;
        if line.len() == 0 {
            break;
        } //End of crates
        for i in 0..line.len() {
            let current_char = line.as_bytes()[i] as char;
            if [' ', '[', ']'].contains(&current_char) {
                continue;
            }
            let stack_index = (i / 4 + 1) as u32;
            match stacks.get_mut(&stack_index) {
                Some(stack) => stack.insert(0, current_char),
                None => {
                    stacks.insert(stack_index, vec![current_char]);
                }
            };
        }
    }
    //move crates
    //dbg!(&stacks);
    for i in index..input_lines.len() {
        let line = &input_lines[i];
        stacks = move_crates(&line, stacks);
    }
    //get top stacks values
    let mut stack_keys: Vec<&u32> = stacks.keys().collect();
    stack_keys.sort();
    let top_stacks: Vec<char> = stack_keys
        .iter()
        .map(|k| stacks.get(k).unwrap()[stacks.get(k).unwrap().len() - 1]) // get the stack and his last character
        .collect();
    return Ok(top_stacks.iter().collect());
}

fn move_crates2(instruction: &str, mut stacks: HashMap<u32, Vec<char>>) -> HashMap<u32, Vec<char>> {
    lazy_static! {
        static ref MOVES_RE: Regex =
            Regex::new(r"move (?P<N>\d+) from (?P<FROM>\d+) to (?P<TO>\d+)").unwrap();
    }
    let captures = MOVES_RE.captures(instruction).unwrap();
    let n = captures
        .name("N")
        .unwrap()
        .as_str()
        .parse::<usize>()
        .unwrap();
    let from_stack_index = captures
        .name("FROM")
        .unwrap()
        .as_str()
        .parse::<u32>()
        .unwrap();
    let to_stack_index = captures
        .name("TO")
        .unwrap()
        .as_str()
        .parse::<u32>()
        .unwrap();

    let mut crates_to_move;
    //BORROWLAND o,O
    {
        let from_stack = stacks.get(&from_stack_index).unwrap();
        crates_to_move = from_stack[from_stack.len() - n..].to_vec();
    }
    {
        let to_stack: &mut Vec<_> = stacks.get_mut(&to_stack_index).unwrap();
        to_stack.append(&mut crates_to_move);
    }
    let from_stack = stacks.get_mut(&from_stack_index).unwrap();
    from_stack.truncate(from_stack.len() - n);
    stacks
}

fn second_question(lines: io::Lines<io::BufReader<File>>) -> io::Result<String> {
    let input_lines: Vec<String> = lines.map(|line| line.unwrap()).collect();
    let mut stacks: HashMap<u32, Vec<char>> = HashMap::new();
    let mut index: usize = 0;
    //parse crates config
    for i in 0..input_lines.len() {
        let line = &input_lines[i];
        index += 1;
        if line.len() == 0 {
            break;
        } //End of crates
        for i in 0..line.len() {
            let current_char = line.as_bytes()[i] as char;
            if [' ', '[', ']'].contains(&current_char) {
                continue;
            }
            let stack_index = (i / 4 + 1) as u32;
            match stacks.get_mut(&stack_index) {
                Some(stack) => stack.insert(0, current_char),
                None => {
                    stacks.insert(stack_index, vec![current_char]);
                }
            };
        }
    }
    //move crates
    //dbg!(&stacks);
    for i in index..input_lines.len() {
        let line = &input_lines[i];
        stacks = move_crates2(&line, stacks);
    }
    //get top stacks values
    let mut stack_keys: Vec<&u32> = stacks.keys().collect();
    stack_keys.sort();
    let top_stacks: Vec<char> = stack_keys
        .iter()
        .map(|k| stacks.get(k).unwrap()[stacks.get(k).unwrap().len() - 1])
        .collect();
    return Ok(top_stacks.iter().collect());
}
