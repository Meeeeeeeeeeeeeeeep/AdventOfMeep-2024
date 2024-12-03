use std::fs::File;
use std::io::{self, BufRead};
use cute::c;

fn genROC(arr: Vec<i32>) -> Vec<i32> {
    return c![arr[i] - arr[i - 1], for i in 1..arr.len()];
}

fn checkValid(arr: Vec<i32>) -> bool {
    //                       decreasing seq. <-|    |-> increasing seq.
    return arr.iter().all(|&x| -3 <= x && x < 0) || arr.iter().all(|&x| 0 < x && x <= 3)
}

fn solve(mode: i32) -> io::Result<()> {
    /* let response = reqwest::blocking::get("https://adventofcode.com/2024/day/2/input");
       let html_content = response.unwrap().text().unwrap();

       I thought I can just scrape the website but...
       "Puzzle inputs differ by user.  Please log in to get your puzzle input."
       :imsadge:
    */

    let mut unsafe_count: i32 = 0;
    let mut safe_count: i32 = 0;

    let file = File::open("input.txt")?;
    let reader = io::BufReader::new(file);

    for line in reader.lines() {
        let line = line?;                               // Handle potential I/O errors.

        let numbers: Vec<i32> = line
            .split_whitespace()
            .map(|s| s.parse::<i32>().unwrap_or(0))     // Handle parsing errors.
            .collect();

        let rOC: Vec<i32> = genROC(numbers.clone());            // rateOfChange

        if checkValid(rOC) {
            safe_count += 1;
        } else {
            if mode == 1 {
                unsafe_count += 1;
            } else {
                let mut buffer_numbers: Vec<i32> = Vec::new();

                for i in 0..numbers.len() {
                    buffer_numbers = numbers.clone();
                    buffer_numbers.remove(i);

                    let rOC: Vec<i32> = genROC(buffer_numbers);

                    if checkValid(rOC) {
                        safe_count += 1;
                        break;
                    }

                    if i == numbers.len() - 1 {
                        unsafe_count += 1;
                    }
                }
            }
        }
    }

    println!("Unsafe Count: {}", unsafe_count);
    println!("Safe Count: {}", safe_count);
    Ok(())
}

fn main() {
    print!("Which part?\n> ");
    let mut pt = String::new();
 
    io::stdin().read_line(&mut pt).unwrap();
 
    match pt.trim() {
        "1" => solve(1).expect("Huh???"),
        "2" => solve(2).expect("Huh???"),
        _   => println!("Coming soon lel"),
    }
}
