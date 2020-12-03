use std::fs::File;
use std::io::{BufRead, BufReader};

fn count(right: usize, skip: bool) -> i32 {
    let filename = "map.txt";
    let mut trees = 0;
    let mut x_pos = 0;

    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);

    println!("Counting trajectory x {} skipping? {}", right, skip);

    for (index, line) in reader.lines().enumerate() {
        let line = line.unwrap();
        if skip && (index % 2) == 1 {
        } else {
            let tile = line.chars().nth(x_pos).unwrap();
            if tile == '#' {
                trees = trees + 1;
            }
            x_pos = (x_pos + right) % line.len();    
        }
    }

    println!("Found {}", trees);
    return trees;
}

fn main() {
    let path1 = count(1, false);
    let path2 = count(3, false);
    let path3 = count(5, false);
    let path4 = count(7, false);
    let path5 = count(1, true);
    let multiplied = path1 * path2 * path3 * path4 * path5;
    println!("Paths: {} {} {} {} {}", path1, path2, path3, path4, path5);
    println!("Multiplied: {}", multiplied);
}
