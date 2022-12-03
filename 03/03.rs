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

fn get_priority(elem: char) -> u32 {
    let ord: u32 = elem.into();
    if ord as i32 - <char as Into<u32>>::into('a') as i32 > 0 {
        return ord - <char as Into<u32>>::into('a') + 1;
    }
    return ord - <char as Into<u32>>::into('A') + 27;
}

fn first_question(lines: io::Lines<io::BufReader<File>>) -> io::Result<u32> {
    let mut prio_sum: u32 = 0;
    for line in lines {
        let l = line?;
        let line_len = l.len();
        let mut first_half_set = HashSet::new();
        first_half_set.extend(l[0..l.len() / 2].chars());

        let mut second_half_set = HashSet::new();
        second_half_set.extend(l[line_len / 2..].chars());
        for c in first_half_set.intersection(&second_half_set) {
            prio_sum += get_priority(*c);
        }
    }
    return Ok(prio_sum);
}

fn second_question(lines: io::Lines<io::BufReader<File>>) -> io::Result<u32> {
    let mut prio_sum: u32 = 0;
    let mut n = 0;
    let mut sets = [HashSet::new(), HashSet::new(), HashSet::new()];
    let sets_len = sets.len();
    for line in lines {
        let l = line?;
        sets[n].extend(l.chars());
        if n == sets_len - 1 {
            let mut accumulator: HashSet<char> = HashSet::new();
            accumulator.extend(sets[0].iter());
            for i in 1..sets_len {
                let mut inter: HashSet<char> = HashSet::new();
                inter.extend(accumulator.intersection(&sets[i]));
                accumulator = inter;
            }
            for c in accumulator.iter() {
                prio_sum += get_priority(*c);
            }
            n = 0;
            sets = [HashSet::new(), HashSet::new(), HashSet::new()];
        } else {
            n += 1;
        }
    }
    return Ok(prio_sum);
}
