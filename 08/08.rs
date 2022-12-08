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

fn first_question(lines: io::Lines<io::BufReader<File>>) -> io::Result<u32> {
    let input_lines: Vec<String> = lines.map(|line| line.unwrap()).collect();
    let mut field: Vec<Vec<u32>> = vec![];
    for line in input_lines {
        let row: Vec<u32> = line.chars().map(|c| c.to_digit(10).unwrap()).collect();
        field.push(row);
    }
    let mut total_visible: u32 = 0;
    for r in 1..field.len() - 1 {
        for c in 1..field.len() - 1 {
            let mut visible = 4;
            let tree = field[r][c];
            for y in 0..r {
                if field[y][c] >= tree {
                    visible -= 1;
                    break;
                }
            }
            for y in r + 1..field.len() {
                if field[y][c] >= tree {
                    visible -= 1;
                    break;
                }
            }
            for x in 0..c {
                if field[r][x] >= tree {
                    visible -= 1;
                    break;
                }
            }
            for x in c + 1..field.len() {
                if field[r][x] >= tree {
                    visible -= 1;
                    break;
                }
            }
            if visible > 0 {
                total_visible += 1
            }
        }
    }

    Ok(total_visible + 4 * field.len() as u32 - 4)
}

fn second_question(lines: io::Lines<io::BufReader<File>>) -> io::Result<u32> {
    let input_lines: Vec<String> = lines.map(|line| line.unwrap()).collect();
    let mut field: Vec<Vec<u32>> = vec![];
    for line in input_lines {
        let row: Vec<u32> = line.chars().map(|c| c.to_digit(10).unwrap()).collect();
        field.push(row);
    }
    let mut tw_max: u32 = 0;
    for r in 1..field.len() - 1 {
        for c in 1..field.len() - 1 {
            let mut tw_l = 0;
            let mut tw_r = 0;
            let mut tw_u = 0;
            let mut tw_d = 0;

            let tree = field[r][c];
            for y in (0..=r - 1).rev() {
                tw_u += 1;
                if field[y][c] >= tree {
                    break;
                }
            }
            for y in r + 1..field.len() {
                tw_d += 1;
                if field[y][c] >= tree {
                    break;
                }
            }
            for x in (0..=c - 1).rev() {
                tw_l += 1;
                if field[r][x] >= tree {
                    break;
                }
            }
            for x in c + 1..field.len() {
                tw_r += 1;
                if field[r][x] >= tree {
                    break;
                }
            }
            if tw_max < tw_l * tw_r * tw_u * tw_d {
                tw_max = tw_l * tw_r * tw_u * tw_d;
            }
        }
    }

    Ok(tw_max)
}
