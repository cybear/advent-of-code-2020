use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let filename = "map.txt";
    let mut trees = 0;
    let mut x_pos = 0;

    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);

    for (index, line) in reader.lines().enumerate() {
        let line = line.unwrap();
        let tile = line.chars().nth(x_pos).unwrap();
        if tile == '#' {
            trees = trees + 1;
        }
        println!("{} line {} xpos {} tile {}", line, index, x_pos, tile);
        x_pos = (x_pos + 3) % line.len();
    }
    println!("Trees: {}", trees);
}
