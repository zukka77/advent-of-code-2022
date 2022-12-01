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
    let mut calories = 0;
    let mut max_calories = 0;
    for line in lines {
        let l = line?;
        if l.len() == 0 {
            if calories > max_calories {
                max_calories = calories;
            }
            calories = 0;
        } else {
            calories += l.parse::<i32>().unwrap();
        }
    }
    return Ok(max_calories);
}

fn second_question(lines: io::Lines<io::BufReader<File>>) -> io::Result<i32> {
    let mut calories = 0;
    let mut max_calories = [0, 0, 0];
    for line in lines {
        let l = line?;
        if l.len() == 0 {
            max_calories.sort();
            for (i, m) in max_calories.iter_mut().enumerate() {
                if calories > *m {
                    max_calories[i] = calories;
                    break;
                }
            }
            calories = 0;
        } else {
            calories += l.parse::<i32>().unwrap();
        }
    }
    return Ok(max_calories.iter().sum());
}
