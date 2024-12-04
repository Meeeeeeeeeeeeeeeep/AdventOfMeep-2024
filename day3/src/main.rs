use std::fs::File;
use std::io::{self, BufRead};
use regex::Regex;
use cute::c;

    /* oh shit bool switch in the middle of regex check?
    // what if i use match?
    // like Python's string.find(), with increments based on how much there is
    // then after i trim the string until after the match, i check if there's do() or don't()
    // then bool flip then voila?
    */

    // Update 1: nvm there's a simpler solution
    // Update 2: WTF IS WRONG WITH THIS CODE???
    // Update 3: FRICK YOU EDGE CASE

fn find_mul(mode: i32) -> i32 {
    let file = File::open("input.txt").unwrap();
    let reader = io::BufReader::new(file);

    let re = Regex::new(r"mul\((\d*),(\d*)\)").unwrap();
    let yeet = Regex::new(r"don't\(\)[\s\S]*?do\(\)").unwrap();
    let frick_u_edge_case = Regex::new(r"don't\(\).*").unwrap();
    
    let mut result: i32 = 0;

    let mut full_string: String = String::new();

    for line in reader.lines() {
        full_string.push_str(line.unwrap().as_str());
    }

    if mode == 2 {
        for x in yeet.captures_iter(&full_string.clone()) {
            full_string = full_string.replace(x.get(0).unwrap().as_str(), "");
        }

        for x in frick_u_edge_case.captures_iter(&full_string.clone()) {
            full_string = full_string.replace(x.get(0).unwrap().as_str(), "");
        }
    }

    for c in re.captures_iter(&full_string) {
        let num1: i32 = c[1].parse().unwrap();
        let num2: i32 = c[2].parse().unwrap();

        result += num1 * num2;
    }

    result
}

fn main() {
    let mut res: i32 = 0;

    print!("Which part?\n> ");

    let mut pt = String::new();
    io::stdin().read_line(&mut pt).unwrap();
 
    match pt.trim() {
        "1" => res = find_mul(1),
        "2" => res = find_mul(2),
        _   => println!("Coming soon lel"),
    }

    println!("{}", res)
}
