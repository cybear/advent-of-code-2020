pub fn calculate(s: &str) -> i32 {
    let numbers: Vec<i32> = s.lines().map(|x| get_seat_id(x)).collect();
    numbers.iter().cloned().fold(0, i32::max)
}

fn get_seat_id(passport: &str) -> i32 {
    let mut c_arr = passport.chars();
    let mut y = 0;
    let mut x = 0;
    // Y
    if c_arr.next().unwrap() == 'B' {y = y + 64;}
    if c_arr.next().unwrap() == 'B' {y = y + 32;}
    if c_arr.next().unwrap() == 'B' {y = y + 16;}
    if c_arr.next().unwrap() == 'B' {y = y + 8;}
    if c_arr.next().unwrap() == 'B' {y = y + 4;}
    if c_arr.next().unwrap() == 'B' {y = y + 2;}
    if c_arr.next().unwrap() == 'B' {y = y + 1;}
    // X
    if c_arr.next().unwrap() == 'R' {x = x + 4;}
    if c_arr.next().unwrap() == 'R' {x = x + 2;}
    if c_arr.next().unwrap() == 'R' {x = x + 1;}

    y * 8 + x
}


#[cfg(test)]
mod tests {
    use super::*;

    static TESTDATA: &str =
"FBFBBFFRLR
BFFFBBFRRR
FFFBBBFRRR
BBFFBBFRLL";

    #[test]
    fn test_get_seat_id() {
        assert_eq!(get_seat_id("FBFBBFFRLR"), 357);
        assert_eq!(get_seat_id("BFFFBBFRRR"), 567);
        assert_eq!(get_seat_id("FFFBBBFRRR"), 119);
        assert_eq!(get_seat_id("BBFFBBFRLL"), 820);
    }

    #[test]
    fn test_get_highest_seat_id() {
        assert_eq!(calculate(TESTDATA), 820);
    }
}
