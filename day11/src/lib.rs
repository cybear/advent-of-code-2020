use std::cmp;
use std::fmt;

#[derive(Debug)]
pub struct Flight {
    pub layout: Vec<Vec<char>>,
    pub x: usize,
    pub y: usize,
}

// Allow println for Flight
impl fmt::Display for Flight {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let layout_arr: Vec<String> = self.layout
            .iter().map(
                |row| row.iter().collect()
            ).collect();
        write!(
            f,
            "Flight: rows {} cols {}\n{}",
            self.y, self.x, layout_arr.join("\n"))
    }
}

// Allow comparison for Flight
impl PartialEq for Flight {
    fn eq(&self, other: &Self) -> bool {
        let flight_x = self.x;
        let flight_y = self.y;
        for y in 0..flight_y {
            for x in 0..flight_x {
                let self_seat = self.layout[y][x];
                let other_seat = other.layout[y][x];
                if self_seat != other_seat {
                    return false;
                }
            }
        }
        true
    }
}

impl Eq for Flight {}

pub fn parse_file(s: &str) -> Flight {
    let layout: Vec<Vec<char>> = s
        .lines()
        .map(|l| parse_line(l))
        .collect();
    let x = layout[0].len();
    let y = layout.len();
    Flight {
        layout,
        x,
        y,
    }
}

pub fn get_adjacent_occupied_seats(flight: &Flight, x: usize, y: usize) -> usize {
    get_adjacent_positions(flight, x, y)
        .iter()
        .filter(
        |position| position == &&'#'
    ).collect::<String>().len()
}

pub fn get_all_occupied_seats(flight: &Flight) -> usize {
    flight.layout.iter().map(
        |row|get_row_occupied_seats(row)
    ).sum() // .collect::<Vec<usize>>()
}

fn get_row_occupied_seats(row: &Vec<char>) -> usize {
    row.iter().filter(|c| **c == '#').collect::<Vec<&char>>().len()
}

fn parse_line(line: &str) -> Vec<char> {
    line
        .chars()
        .map(|x| match x {
            '.' | 'L' | '#' => x,
            _ => panic!("Invalid character"),
        })
        .collect()
}

fn get_adjacent_positions(flight: &Flight, x:usize, y:usize) -> Vec<char> {
    let mut adjacent_seats: Vec<char> = Vec::new();
    let y_min = cmp::max(0, y.saturating_sub(1));
    let y_max = cmp::min(flight.y, y+2);
    let x_min = cmp::max(0, x.saturating_sub(1));
    let x_max = cmp::min(flight.x, x+2);
    for y_pos in y_min..y_max {
        for x_pos in x_min..x_max {
            let is_me = x_pos == x && y_pos == y;
            if !is_me {
                adjacent_seats.push(flight.layout[y_pos][x_pos]);
            }
        }
    }
    adjacent_seats
}

fn look_chair_up(flight: &Flight, x:usize, y:usize) -> char {
    if y == 0 {return 'X';}
    let mut y_pos = y - 1;
    while y_pos > 0 {
        let pos = flight.layout[y_pos][x];
        if pos == '#' || pos == 'L' {
            return pos;
        }
        y_pos -= 1;
    }
    'X' // Did not find
}
fn look_chair_down(flight: &Flight, x:usize, y:usize) -> char {
    if y >= flight.y {return 'X';}
    let y_start = y + 1;
    for y_pos in y_start..flight.y {
        let pos = flight.layout[y_pos][x];
        if pos == '#' || pos == 'L' {
            return pos;
        }
    }
    'X' // Did not find
}
fn look_chair_left(flight: &Flight, x:usize, y:usize) -> char {
    if x == 0 {return 'X';}
    let mut x_pos = x - 1;
    while x_pos > 0 {
        let pos = flight.layout[y][x_pos];
        if pos == '#' || pos == 'L' {
            return pos;
        }
        x_pos -= 1;
    }
    'X' // Did not find
}
fn look_chair_right(flight: &Flight, x:usize, y:usize) -> char {
    if x >= flight.x {return 'X';}
    let x_start = x + 1;
    for x_pos in x_start..flight.x {
        let pos = flight.layout[y][x_pos];
        if pos == '#' || pos == 'L' {
            return pos;
        }
    }
    'X' // Did not find
}
fn look_chair_up_left(flight: &Flight, x:usize, y:usize) -> char {
    if x == 0 || y == 0 {return 'X';}
    let mut x_pos = x - 1;
    let mut y_pos = y - 1;
    while x_pos >= 0 && y_pos >= 0 {
        let pos = flight.layout[y_pos][x_pos];
        if pos == '#' || pos == 'L' {
            return pos;
        }
        if x_pos == 0 || y_pos == 0 {return 'X';}
        x_pos -= 1;
        y_pos -= 1;
    }
    'X' // Did not find
}
fn look_chair_down_right(flight: &Flight, x:usize, y:usize) -> char {
    if x >= flight.x || y >= flight.y {return 'X';}
    let mut x_pos = x + 1;
    let mut y_pos = y + 1;
    while x_pos < flight.x && y_pos < flight.y {
        let pos = flight.layout[y_pos][x_pos];
        if pos == '#' || pos == 'L' {
            return pos;
        }
        x_pos += 1;
        y_pos += 1;
    }
    'X' // Did not find
}
fn look_chair_up_right(flight: &Flight, x:usize, y:usize) -> char {
    if x >= flight.x || y == 0 {return 'X';}
    let mut x_pos = x + 1;
    let mut y_pos = y - 1;
    while x_pos < flight.x && y_pos >= 0 {
        // println!("{} {}", x_pos, y_pos);
        let pos = flight.layout[y_pos][x_pos];
        if pos == '#' || pos == 'L' {
            return pos;
        }
        if y_pos == 0 {return 'X';}
        x_pos += 1;
        y_pos -= 1;
    }
    'X' // Did not find
}
fn look_chair_down_left(flight: &Flight, x:usize, y:usize) -> char {
    if x == 0 || y >= flight.y {return 'X';}
    let mut x_pos = x - 1;
    let mut y_pos = y + 1;
    while x_pos >= 0 && y_pos < flight.y {
        let pos = flight.layout[y_pos][x_pos];
        if pos == '#' || pos == 'L' {
            return pos;
        }
        if x_pos == 0 {return 'X';}
        x_pos -= 1;
        y_pos += 1;
    }
    'X' // Did not find
}

pub fn look_all_chairs(flight: &Flight, x:usize, y:usize) -> usize {
    let mut sum = 0;
    if '#' == look_chair_up(flight, x, y) { sum += 1; }
    if '#' == look_chair_up_right(flight, x, y) { sum += 1; }
    if '#' == look_chair_right(flight, x, y) { sum += 1; }
    if '#' == look_chair_down_right(flight, x, y) { sum += 1; }
    if '#' == look_chair_down(flight, x, y) { sum += 1; }
    if '#' == look_chair_down_left(flight, x, y) { sum += 1; }
    if '#' == look_chair_left(flight, x, y) { sum += 1; }
    if '#' == look_chair_up_left(flight, x, y) { sum += 1; }
    sum
}



#[cfg(test)]
mod tests {
    use super::*;

    static TESTDATA: &str = "L.LL.LL.LL
LLLLLLL.LL
L.L.L..L..
LLLL.LL.LL
L.LL.LL.LL
L.LLLLL.LL
..L.L.....
LLLLLLLLLL
L.LLLLLL.L
L.LLLLL.LL";

    #[test]
    fn test_parser() {
        let parsed = parse_file(TESTDATA);
        let expected: Vec<Vec<char>> = include_str!("tests/layout1.txt").lines().map(
        |x|x.chars().collect()
    ).collect();
        println!("Parsed {:?}", parsed);
        assert_eq!(parsed.layout, expected);
    }
    #[test]
    fn test_get_adjacent_seats() {
        let flight = parse_file(TESTDATA);
        println!("{}", flight);
        assert_eq!(
            get_adjacent_positions(&flight, 1, 1),
            vec!['L','.','L','L','L','L','.','L']
        );
        assert_eq!(
            get_adjacent_positions(&flight, 0, 0),
            vec!['.','L','L'], "Top left"
        );
        assert_eq!(
            get_adjacent_positions(&flight, flight.y - 1, flight.x - 1),
            vec!['.','L','L'], "Bottom right"
        );
    }
    #[test]
    fn test_get_adjacent_occupied_seats() {
        let flight = parse_file(TESTDATA);
        println!("{}", flight);
        assert_eq!(get_adjacent_occupied_seats(&flight, 1, 1), 0);
        assert_eq!(get_adjacent_occupied_seats(&flight, 0, 0), 0);
        assert_eq!(get_adjacent_occupied_seats(&flight, flight.y - 1, flight.x - 1), 0);
    }
    #[test]
    fn test_get_adjacent_occupied_seats_2() {
        let flight = parse_file(include_str!("tests/layout2.txt"));
        println!("{}", flight);
        assert_eq!(get_adjacent_occupied_seats(&flight, 1, 1), 6);
        assert_eq!(get_adjacent_occupied_seats(&flight, 0, 0), 2);
        assert_eq!(get_adjacent_occupied_seats(&flight, flight.y - 1, flight.x - 1), 2);
    }
    #[test]
    fn test_eq() {
        let parsed = parse_file(TESTDATA);
        let parsed_same = parse_file(TESTDATA);
        let parsed_other = parse_file(include_str!("tests/layout2.txt"));
        println!("Parsed {:?}", parsed);
        assert_eq!(parsed == parsed_same, true);
        assert_eq!(parsed == parsed_other, false);
    }
/*******************************************************************************/
    #[test]
    fn test_look_chair_up() {
        let flight = parse_file(include_str!("tests/2example1.txt"));
        assert_eq!(look_chair_up(&flight, 0, 0), 'X');
        assert_eq!(look_chair_up(&flight, 0, 9), '#');
        assert_eq!(look_chair_up(&flight, 3, 8), 'L');
    }
    #[test]
    fn test_look_chair_down() {
        let flight = parse_file(include_str!("tests/2example1.txt"));
        assert_eq!(look_chair_down(&flight, 3, 9), 'X');
        assert_eq!(look_chair_down(&flight, 0, 0), '#');
        assert_eq!(look_chair_down(&flight, 3, 4), '#');
    }
    #[test]
    fn test_look_chair_left() {
        let flight = parse_file(include_str!("tests/2example1.txt"));
        assert_eq!(look_chair_left(&flight, 3, 8), 'X');
        assert_eq!(look_chair_left(&flight, 0, 0), 'X');
        assert_eq!(look_chair_left(&flight, 5, 4), 'L');
    }
    #[test]
    fn test_look_all_directions() {
        let flight = parse_file(include_str!("tests/2example1.txt"));
        assert_eq!(look_chair_up(&flight, 3, 4), '#', "up");
        assert_eq!(look_chair_down(&flight, 3, 4), '#', "down");
        assert_eq!(look_chair_left(&flight, 3, 4), '#', "left");
        assert_eq!(look_chair_right(&flight, 3, 4), '#', "right");
        assert_eq!(look_chair_up_left(&flight, 3, 4), '#', "up left");
        assert_eq!(look_chair_up_right(&flight, 3, 4), '#', "up right");
        assert_eq!(look_chair_down_left(&flight, 3, 4), '#', "down left");
        assert_eq!(look_chair_down_right(&flight, 3, 4), '#', "down right");
    }
    #[test]
    fn test_look_all_chairs1() {
        let flight = parse_file(include_str!("tests/2example1.txt"));
        assert_eq!(look_all_chairs(&flight, 3, 4), 8);
    }
    fn test_look_all_chairs2() {
        let flight = parse_file(include_str!("tests/2example2.txt"));
        assert_eq!(look_all_chairs(&flight, 1, 1), 1);
    }
    fn test_look_all_chairs3() {
        let flight = parse_file(include_str!("tests/2example3.txt"));
        assert_eq!(look_all_chairs(&flight, 3, 3), 0);
    }
}
