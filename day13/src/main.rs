mod lib;

fn main() {
    let data = lib::Notes::new(include_str!("input.txt"));
    println!("{}", data);
    let result = lib::solve(data);
    println!("Day 13 part 1: {}", result);
}
