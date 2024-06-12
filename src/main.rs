use std::{env, fs};
use onig::Regex;

fn main() {
    let args: Vec<_> = env::args().collect();

    let file_path = args.get(2).map_or("input.txt".to_string(), |s| s.to_owned());
    let input = fs::read_to_string(file_path).unwrap();

    let task = args.get(1).map_or(1,  |x| x.parse().unwrap());

    let result = match task {
        1 => task1(&input),
        2 => task2(&input),
        _ => panic!("invalid task")
    };

    println!("{}", result)
}

fn task1(input: &str) -> i32 {
    let lines = input.split('\n');
    let results: Vec<_> = lines.into_iter().map(|x| {
        let c1 = x.chars().reduce(|acc, e| {
            if acc.is_numeric() { acc } else { e }
        }).unwrap();
        let c2 = x.chars().rev().reduce(|acc, e| {
            if acc.is_numeric() { acc } else { e }
        }).unwrap();
        String::from_iter(vec![c1, c2])
    }).collect();

    results.iter().map(|x| x.parse::<i32>().unwrap()).sum()
}

fn task2(input: &str) -> i32 {
    let mut lines = input.split('\n');
    // let res: Vec<_>  = "abcabca".match_indices("abca").collect();
    let res: Vec<_> = Regex::new("abca").unwrap().find_iter("abcabca").collect();
    println!("{:?}", res);
    todo!()
}
