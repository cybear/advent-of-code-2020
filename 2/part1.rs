use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn validate(data: &String) -> bool {
    let mut iter = data.split_whitespace();
    let mut range_iter = iter.next().unwrap().split("-");
    let min_length: i32 = range_iter.next().unwrap().parse().unwrap();
    let max_length: i32 = range_iter.next().unwrap().parse().unwrap();
    let required_char = iter.next().unwrap().chars().nth(0).unwrap();
    let password = iter.next().unwrap().chars();

    let mut n = 0;
    for c in password {
        if c == required_char {
            n = n + 1;
        }
    }
    if n <= max_length && n >= min_length {
    } else {
        println!("req letter {}, req length {}-{}, found {}", required_char, min_length, max_length, n);
    }
    return n <= max_length && n >= min_length;
}

fn main() {
    let mut sum = 0;
    // File hosts must exist in current path before this produces output
    if let Ok(lines) = read_lines("./data.txt") {
        // Consumes the iterator, returns an (Optional) String
        for line in lines {
            if let Ok(data) = line {
                let result = validate(&data);
                if result {
                    sum = sum + 1;
                }
                // return;
                // println!("{}", data);
            }
        }
        println!("valid ones {}", sum);
    }
    
}

// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}