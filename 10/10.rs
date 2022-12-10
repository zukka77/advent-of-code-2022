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

fn first_question(lines: io::Lines<io::BufReader<File>>) -> io::Result<i32> {
    let input_lines: Vec<String> = lines.map(|line| line.unwrap()).collect();
    #[allow(non_snake_case)]
    let mut X = 1;
    let mut strenghts: Vec<i32> = vec![];
    let mut instruction_queue = vec![];
    let mut cycle = 0;
    for (i, line) in input_lines.iter().enumerate() {
        if instruction_queue.len() > 0 {
            let val: Vec<i32> = instruction_queue.drain(0..1).collect();
            X += val[0];
        }
        cycle = i as i32 + 1;
        if cycle == 20 || (cycle - 20) % 40 == 0 && cycle <= 220 {
            strenghts.push(cycle * X);
        }
        if line == "noop" {
            instruction_queue.push(0);
        } else {
            let quantity = line.split(" ").collect::<Vec<&str>>()[1]
                .parse::<i32>()
                .unwrap();
            instruction_queue.extend_from_slice(&[0, quantity]);
        }
    }
    // DRAIN QUEUE
    while instruction_queue.len() > 0 {
        let val: Vec<i32> = instruction_queue.drain(0..1).collect();
        X += val[0];
        cycle += 1;
        if cycle == 20 || (cycle - 20) % 40 == 0 && cycle <= 220 {
            strenghts.push(cycle * X);
        }
    }

    Ok(strenghts.iter().sum())
}

fn second_question(lines: io::Lines<io::BufReader<File>>) -> io::Result<String> {
    let input_lines: Vec<String> = lines.map(|line| line.unwrap()).collect();
    #[allow(non_snake_case)]
    let mut X = 1;
    let mut instruction_queue = vec![];
    let mut cycle = 0;
    for (i, line) in input_lines.iter().enumerate() {
        if cycle % 40 == 0 {
            println!("");
        }
        if cycle / 40 == 6 {
            break;
        }
        if instruction_queue.len() > 0 {
            let val: Vec<i32> = instruction_queue.drain(0..1).collect();
            X += val[0];
        }
        cycle = i as i32 + 1;
        if ((cycle - 1) % 40 - X).abs() <= 1 {
            print!("#");
        } else {
            print!(".");
        }
        if line == "noop" {
            instruction_queue.push(0);
        } else {
            let quantity = line.split(" ").collect::<Vec<&str>>()[1]
                .parse::<i32>()
                .unwrap();
            instruction_queue.extend_from_slice(&[0, quantity]);
        }
    }
    // DRAIN QUEUE
    while instruction_queue.len() > 0 && cycle / 40 < 6 {
        if cycle % 40 == 0 {
            println!("");
        }
        let val: Vec<i32> = instruction_queue.drain(0..1).collect();
        X += val[0];
        cycle += 1;
        if ((cycle - 1) % 40 - X).abs() <= 1 {
            print!("#");
        } else {
            print!(".");
        }
    }
    Ok("".to_string())
}
