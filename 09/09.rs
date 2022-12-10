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

fn move_tail<'a>(
    head_position: &mut Vec<i32>,
    tail_position: &mut Vec<i32>,
    tail_positions: &'a mut HashSet<(i32, i32)>,
) {
    if head_position[1] - tail_position[1] > 1 {
        //UP
        if head_position[0] == tail_position[0] {
            tail_position[1] += 1;
        } else if head_position[0] > tail_position[0] {
            tail_position[1] += 1;
            tail_position[0] += 1;
        } else {
            tail_position[1] += 1;
            tail_position[0] -= 1;
        }
    }
    if head_position[1] - tail_position[1] < -1 {
        //DOWN
        if head_position[0] == tail_position[0] {
            tail_position[1] -= 1;
        } else if head_position[0] > tail_position[0] {
            tail_position[1] -= 1;
            tail_position[0] += 1;
        } else {
            tail_position[1] -= 1;
            tail_position[0] -= 1;
        }
    }
    if head_position[0] - tail_position[0] > 1 {
        //RIGHT
        if head_position[1] == tail_position[1] {
            tail_position[0] += 1;
        } else if head_position[1] > tail_position[1] {
            tail_position[0] += 1;
            tail_position[1] += 1;
        } else {
            tail_position[0] += 1;
            tail_position[1] -= 1;
        }
    }
    if head_position[0] - tail_position[0] < -1 {
        //LEFT
        if head_position[1] == tail_position[1] {
            tail_position[0] -= 1;
        } else if head_position[1] > tail_position[1] {
            tail_position[0] -= 1;
            tail_position[1] += 1;
        } else {
            tail_position[0] -= 1;
            tail_position[1] -= 1;
        }
    }
    tail_positions.insert((tail_position[0], tail_position[1]));
}

fn move_up<'a>(
    head_position: &mut Vec<i32>,
    tail_position: &mut Vec<i32>,
    steps: u32,
    tail_positions: &'a mut HashSet<(i32, i32)>,
) {
    for _ in 0..steps {
        head_position[1] += 1;
        move_tail(head_position, tail_position, tail_positions);
    }
}

fn move_down<'a>(
    head_position: &mut Vec<i32>,
    tail_position: &mut Vec<i32>,
    steps: u32,
    tail_positions: &'a mut HashSet<(i32, i32)>,
) {
    for _ in 0..steps {
        head_position[1] -= 1;
        move_tail(head_position, tail_position, tail_positions);
    }
}

fn move_right<'a>(
    head_position: &mut Vec<i32>,
    tail_position: &mut Vec<i32>,
    steps: u32,
    tail_positions: &'a mut HashSet<(i32, i32)>,
) {
    for _ in 0..steps {
        head_position[0] += 1;
        move_tail(head_position, tail_position, tail_positions);
    }
}

fn move_left<'a>(
    head_position: &mut Vec<i32>,
    tail_position: &mut Vec<i32>,
    steps: u32,
    tail_positions: &'a mut HashSet<(i32, i32)>,
) {
    for _ in 0..steps {
        head_position[0] -= 1;
        move_tail(head_position, tail_position, tail_positions);
    }
}

fn first_question(lines: io::Lines<io::BufReader<File>>) -> io::Result<usize> {
    let input_lines: Vec<String> = lines.map(|line| line.unwrap()).collect();
    let mut head_position = vec![0, 0];
    let mut tail_position = vec![0, 0];
    let mut tail_positions = HashSet::from([(0, 0)]);
    //tail_positions.insert("0_0".to_string());
    for line in input_lines {
        let parts: Vec<&str> = line.split(" ").collect();
        let direction = parts[0];
        let steps = parts[1].parse::<u32>().unwrap();
        match direction {
            "U" => move_up(
                &mut head_position,
                &mut tail_position,
                steps,
                &mut tail_positions,
            ),
            "D" => move_down(
                &mut head_position,
                &mut tail_position,
                steps,
                &mut tail_positions,
            ),
            "R" => move_right(
                &mut head_position,
                &mut tail_position,
                steps,
                &mut tail_positions,
            ),
            "L" => move_left(
                &mut head_position,
                &mut tail_position,
                steps,
                &mut tail_positions,
            ),
            _ => unreachable!("WRONG MOVE"),
        };
    }
    Ok(tail_positions.len())
}

fn second_question(lines: io::Lines<io::BufReader<File>>) -> io::Result<usize> {
    let input_lines: Vec<String> = lines.map(|line| line.unwrap()).collect();
    let mut rope_positions: Vec<Vec<i32>> = vec![];
    let mut tail_positions = vec![];
    for _ in 0..10 {
        //init structures
        rope_positions.push(vec![0, 0]);
        tail_positions.push(HashSet::from([(0, 0)]));
    }
    for line in input_lines {
        let parts: Vec<&str> = line.split(" ").collect();
        let direction = parts[0];
        let steps = parts[1].parse::<u32>().unwrap();
        for _ in 0..steps {
            let splits = rope_positions.split_at_mut(1);
            let mut head_position = &mut splits.0[0];
            let mut tail_position = &mut splits.1[0];
            match direction {
                "U" => move_up(
                    &mut head_position,
                    &mut tail_position,
                    1,
                    &mut tail_positions[1],
                ),
                "D" => move_down(
                    &mut head_position,
                    &mut tail_position,
                    1,
                    &mut tail_positions[1],
                ),
                "R" => move_right(
                    &mut head_position,
                    &mut tail_position,
                    1,
                    &mut tail_positions[1],
                ),
                "L" => move_left(
                    &mut head_position,
                    &mut tail_position,
                    1,
                    &mut tail_positions[1],
                ),
                _ => unreachable!("WRONG MOVE"),
            };
            for k in 1..rope_positions.len() - 1 {
                let splits = rope_positions.split_at_mut(k + 1);
                let mut head_position = &mut splits.0[k];
                let mut tail_position = &mut splits.1[0];
                move_tail(
                    &mut head_position,
                    &mut tail_position,
                    &mut tail_positions[k + 1],
                );
            }
        }
    }

    Ok(tail_positions[tail_positions.len() - 1].len())
}
