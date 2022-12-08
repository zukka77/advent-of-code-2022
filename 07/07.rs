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

fn first_question(lines: io::Lines<io::BufReader<File>>) -> io::Result<u32> {
    let input_lines: Vec<String> = lines.map(|line| line.unwrap()).collect();
    let mut cwd = String::from("/");
    let mut dir_sizes: HashMap<String, u32> = HashMap::new();

    for line in input_lines {
        if line.len() == 0 {
            continue;
        }
        if line.starts_with("$ cd") {
            let d = line.split(" ").collect::<Vec<&str>>()[2];
            if d == "/" {
                cwd = "/".to_string();
            } else if d == ".." {
                let tmp = cwd.split("/").collect::<Vec<&str>>();
                cwd = tmp[0..tmp.len() - 1].join("/");
            } else {
                if cwd == "/" {
                    cwd = ["/".to_string(), d.to_string()].concat();
                } else {
                    cwd = [cwd, "/".to_string(), d.to_string()].concat();
                }
            }
            dir_sizes.entry(cwd.clone()).or_insert(0);
        } else if line.starts_with("$ ls") || line.starts_with("dir") {
            continue;
        } else {
            let size = line.split(" ").collect::<Vec<&str>>()[0]
                .parse::<u32>()
                .unwrap();
            dir_sizes
                .entry(cwd.clone())
                .and_modify(|val| *val += size)
                .or_insert(size);
        }
    }

    let mut total_sizes: HashMap<String, u32> = HashMap::new();
    for d in dir_sizes.keys() {
        total_sizes.insert(
            d.to_string(),
            dir_sizes
                .iter()
                .filter(|(k, _)| k.starts_with(d))
                .map(|(_, v)| v)
                .sum(),
        );
    }

    Ok(total_sizes
        .iter()
        .map(|(_, v)| v)
        .filter(|v| **v <= 100_000)
        .sum())
}

fn second_question(lines: io::Lines<io::BufReader<File>>) -> io::Result<u32> {
    let input_lines: Vec<String> = lines.map(|line| line.unwrap()).collect();

    let mut cwd = String::from("/");
    let mut dir_sizes: HashMap<String, u32> = HashMap::new();

    for line in input_lines {
        if line.len() == 0 {
            continue;
        }
        if line.starts_with("$ cd") {
            let d = line.split(" ").collect::<Vec<&str>>()[2];
            if d == "/" {
                cwd = "/".to_string();
            } else if d == ".." {
                let tmp = cwd.split("/").collect::<Vec<&str>>();
                cwd = tmp[0..tmp.len() - 1].join("/");
            } else {
                if cwd == "/" {
                    cwd = ["/".to_string(), d.to_string()].concat();
                } else {
                    cwd = [cwd, "/".to_string(), d.to_string()].concat();
                }
            }
            dir_sizes.entry(cwd.clone()).or_insert(0); //dir with no files
        } else if line.starts_with("$ ls") || line.starts_with("dir") {
            continue;
        } else {
            let size = line.split(" ").collect::<Vec<&str>>()[0]
                .parse::<u32>()
                .unwrap();
            dir_sizes
                .entry(cwd.clone())
                .and_modify(|val| *val += size)
                .or_insert(size);
        }
    }

    let mut total_sizes: HashMap<String, u32> = HashMap::new();
    for d in dir_sizes.keys() {
        total_sizes.insert(
            d.to_string(),
            dir_sizes
                .iter()
                .filter(|(k, _)| k.starts_with(d))
                .map(|(_, v)| v)
                .sum(),
        );
    }
    let needed_space = 30_000_000 - (70_000_000 - total_sizes["/"]);

    let mut good_dirs = total_sizes
        .iter()
        .map(|(_, v)| v)
        .filter(|v| **v >= needed_space)
        .collect::<Vec<&u32>>();
    good_dirs.sort();
    Ok(*good_dirs[0])
}
