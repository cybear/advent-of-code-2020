mod part1;
// mod part2;
mod lib;

fn main() {
    let data = lib::parse_file(include_str!("input.txt"));
    let solution1 = part1::solve(data);
    println!("Day 10 part 1: {}", solution1);
    // let solution2 = part2::solve(data);
    // println!("Day 10 part 2: {}", solution2);
    
}
