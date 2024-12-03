use std::collections::BTreeSet;
use std::fs::File;
use std::io::{self, BufRead};
use cute::c;

fn prepare_list() -> (Vec<i64>, Vec<i64>) {
    let mut list_num1: Vec<i64> = Vec::new();
    let mut list_num2: Vec<i64> = Vec::new();

    let file = File::open("input.txt").unwrap();
    let reader = io::BufReader::new(file);

    for line in reader.lines() {
        let line = line.unwrap();

        let numbers: Vec<i64> = line
            .split_whitespace()
            .map(|s| s.parse::<i64>().unwrap_or(0))
            .collect();

        list_num1.push(numbers[0]);
        list_num2.push(numbers[1]);
    }

    (list_num1, list_num2)
}

fn main() {
    let (mut left, mut right) = prepare_list();
    let mut res: i64 = 0;

    print!("Which part?\n> ");
    let mut pt = String::new();
 
    io::stdin().read_line(&mut pt).unwrap();
    
    if pt.trim() == "1" {
        left.sort();
        right.sort();

        res = left.iter().zip(right.iter()).map(|(l, r)| (l - r).abs()).sum();
    } else if pt.trim() == "2" {
        let set_left: BTreeSet<i64> = BTreeSet::from_iter(left.clone());
        let set_right: BTreeSet<i64> = BTreeSet::from_iter(right.clone());
        let mut buf: i64;

        for unique_item in set_left {
            buf = unique_item 
                * left.iter().filter(|&&x| x == unique_item).count() as i64
                * right.iter().filter(|&&x| x == unique_item).count() as i64;
            
            res += buf;
        }
    } else {
        res = 0;
    }
    
    println!("Result: {}\nIf it's 0, then most probably it's an error lel", res);
}
