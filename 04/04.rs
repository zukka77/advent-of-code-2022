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
    let mut contained_ranges: u32 = 0;
    for line in lines {
        let l = line?;
        let ranges: Vec<Vec<u32>> = l
            .split(",")
            .map(|range| {
                range
                    .split("-")
                    .map(|value| value.parse::<u32>().unwrap())
                    .collect()
            })
            .collect();
        let first_range = &ranges[0];
        let second_range = &ranges[1];
        if first_range[0] >= second_range[0] && first_range[1] <= second_range[1]
            || first_range[0] <= second_range[0] && first_range[1] >= second_range[1]
        {
            contained_ranges += 1;
        }
    }

    return Ok(contained_ranges);
}

fn second_question(lines: io::Lines<io::BufReader<File>>) -> io::Result<u32> {
    let mut overlaps: u32 = 0;
    for line in lines {
        let l = line?;
        let ranges: Vec<Vec<u32>> = l
            .split(",")
            .map(|range| {
                range
                    .split("-")
                    .map(|value| value.parse::<u32>().unwrap())
                    .collect()
            })
            .collect();
        let first_range = &ranges[0];
        let second_range = &ranges[1];
        if first_range[0] <= second_range[0]
            && first_range[1] <= second_range[1]
            && first_range[1] >= second_range[0]
            || first_range[0] >= second_range[0]
                && first_range[0] <= second_range[1]
                && first_range[1] >= second_range[1]
            || first_range[0] >= second_range[0] && first_range[1] <= second_range[1]
            || first_range[0] <= second_range[0] && first_range[1] >= second_range[1]
        {
            overlaps += 1;
        }
    }
    return Ok(overlaps);
}
