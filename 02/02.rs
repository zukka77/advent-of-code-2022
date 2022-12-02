use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

use phf::phf_map;

static POINTS: phf::Map<&'static str, i32> = phf_map!(
    "ROCK" => 1,
    "PAPER" => 2,
    "SCISSORS" => 3,
    "X" => 1,
    "Y" => 2,
    "Z" => 3,
);

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
    let mut points: i32 = 0;
    for line in lines {
        let l = line?;
        let moves: Vec<&str> = l.split_whitespace().collect();
        let opponent = moves[0];
        let your = moves[1];
        let point = POINTS.get(moves[1]).unwrap();
        if (opponent == "A" && your == "X")
            || (opponent == "B" && your == "Y")
            || (opponent == "C" && your == "Z")
        {
            points += 3 + point;
            continue;
        }
        if (opponent == "A" && your == "Z")
            || (opponent == "B" && your == "X")
            || (opponent == "C" && your == "Y")
        {
            points += point;
            continue;
        }
        points += 6 + point;
    }
    return Ok(points);
}

fn second_question(lines: io::Lines<io::BufReader<File>>) -> io::Result<i32> {
    let mut points: i32 = 0;
    for line in lines {
        let l = line?;
        let moves: Vec<&str> = l.split_whitespace().collect();
        let opponent = moves[0];
        let outcome = moves[1];
        if outcome == "X" {
            // DEFEAT
            if opponent == "A" {
                //ROCK
                points += POINTS["SCISSORS"];
                continue;
            }
            if opponent == "B" {
                // PAPER
                points += POINTS["ROCK"];
                continue;
            }
            points += POINTS["PAPER"];
            continue;
        }
        if outcome == "Y" {
            //DRAW
            if opponent == "A" {
                points += 3 + POINTS["ROCK"];
                continue;
            }
            if opponent == "B" {
                // PAPER
                points += 3 + POINTS["PAPER"];
                continue;
            }
            points += 3 + POINTS["SCISSORS"];
            continue;
        }
        // WIN
        if opponent == "A" {
            // ROCK
            points += 6 + POINTS["PAPER"];
            continue;
        }
        if opponent == "B" {
            //PAPER
            points += 6 + POINTS["SCISSORS"];
            continue;
        }
        points += 6 + POINTS["ROCK"];
    }
    return Ok(points);
}
