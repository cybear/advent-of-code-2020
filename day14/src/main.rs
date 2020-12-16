mod lib;

fn main() {
    let data = lib::parse_file(include_str!("input.txt"));
    // println!("{}", data);
    let result = lib::solve(data);
    println!("Day 14 part 1: {}", result);
}

