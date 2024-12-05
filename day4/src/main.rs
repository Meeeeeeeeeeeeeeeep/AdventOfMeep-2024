use std::fs::File;
use std::io::{self, BufRead};
use cute::c;

/*----------------
|| NOTE TO SELF ||
------------------
>> IT'S A MISTAKE TO THINK OF THE 2D SPACE HERE WITH CARTESIAN COORDINATES
>> The real format is (row, column)...
>> ... with [row] going positive to the right (like <x -> /inf>),
>> ... and [column] going positive to the bottom (like <y -> -/inf>) */

fn pt_1(grid: &[Vec<char>], word: &str) -> usize {
    let rows = grid.len();
    let cols = grid[0].len();
    let word_chars: Vec<char> = word.chars().collect();
    let mut count = 0;

    // all 8 directions (dx, dy)
    let directions = [
        (0, -1),   // N
        (0, 1),  // S
        (1, 0),   // E
        (-1, 0),  // W
        (1, -1),   // NE
        (-1, -1),  // NW
        (1, 1),  // SE
        (-1, -1), // SW
    ];

    // at (x, y) going to (dx, dy) one direction, if there's a word there then true
    fn is_word_at(grid: &[Vec<char>], word_chars: &[char], x: usize, y: usize, dx: isize, dy: isize) -> bool {
        let rows = grid.len();
        let cols = grid[0].len();

        for (i, &ch) in word_chars.iter().enumerate() {
            let nx = x as isize + i as isize * dx;
            let ny = y as isize + i as isize * dy;

            if nx < 0 || ny < 0 || nx >= rows as isize || ny >= cols as isize {
                return false;
            }

            if grid[nx as usize][ny as usize] != ch {
                return false;
            }
        }
        
        true
    }

    for x in 0..rows {
        for y in 0..cols {
            for &(dx, dy) in &directions {
                if is_word_at(grid, &word_chars, x, y, dx, dy) {
                    count += 1;
                }
            }
        }
    }

    count
}

fn pt_2(grid: &[Vec<char>]) -> usize {
    let rows = grid.len();
    let cols = grid[0].len();
    let mut count = 0;

    //==================================================// Patterns for "X-MAS"
    let patterns = [                                    //--------------------------------
        /* 1. Vertical (default)          */            // (1)      (2)
        [(-1, -1), (-1, 1), (1, 1), (1, -1)],           // M . M    S . S       
        /* 2. Vertical (flipped)          */            // . A .    . A .
        [(-1, 1), (1, -1), (-1, -1), (1, 1)],           // S . S    M . M
        /* 3. Horizontal (default)        */            //
        [(-1, -1), (1, -1), (1, 1), (-1, 1)],           // (3)      (4)
        /* 4. Horizontal (flipped)        */            // M . S    S . M
        [(1, 1), (-1, 1), (-1, -1), (1, -1)],           // . A .    . A .
    ];                                                  // M . S    S . M

    //==================================================// To make sure I'm not lost...
    for x in 1..rows - 1 {                              // We iterate rows besides the first and last,
        for y in 1..cols - 1 {                          // make it 2D with columns besides the first and last,
            for &pattern in &patterns {                 // then iterate through patterns to check (x, y).
                if grid[x][y] == 'A' {                  // If it's 'A'...
                    let mut valid: bool = true;         //
                                                        //
                    for (i, &(dx, dy)) in pattern.iter().enumerate() {
                        let nx = x as isize + dx;       // ... look through the nth pattern in the list and apply (x + dx, y + dy)
                        let ny = y as isize + dy;       // Like, to look around the A center,
                                                        // for its butterfly wings for M and S in the right place.
                        if nx < 0                       // After that, we check if X is out of bounds to the negatives...
                            || ny < 0                   // ... or Y is out of bounds to the negatives...
                            || nx >= rows as isize      // ... or X is out of bounds more than the upper size limit ...
                            || ny >= cols as isize      // ... or Y is out of bounds more than the upper size limit ...
                            || grid[nx as usize][ny as usize] != match i {
                                0 | 3 => 'M',           // ... or if pattern[0] or pattern[3] are not M...
                                1 | 2 => 'S',           // ... or pattern[1] or pattern[2] are not S,...
                                _ => unreachable!(),    //          ---just for error-handling---
                            } {                         //
                            valid = false;              // ...then it's not a valid XMAS pattern,
                            break;                      // and we can just leave the loop early to continue to the next letter.
                        }
                    }

                    if valid {                          // And since the default state for the letters is valid,
                        count += 1;                     // If there's a matching pattern then we can just add 1 to the count.
                    }
                }
            }
        }
    }

    count
}

fn main() {
    let file = File::open("input.txt").unwrap();
    let reader = io::BufReader::new(file);

    let word_search: Vec<Vec<char>> = reader.lines()
                                            .map(|line| line.unwrap().chars().collect())
                                            .collect();
    let word: &str = "XMAS";

    print!("Which part?\n> ");
    let mut pt = String::new();
    io::stdin().read_line(&mut pt).unwrap();

    let count: i32 = match pt.trim() {
        "1" => pt_1(&word_search, word).try_into().unwrap(),
        "2" => pt_2(&word_search).try_into().unwrap(),
         _  => 0,
    };

    println!("Result: {}\n(If it's 0, then most probably it's an error lel)", count);
}
