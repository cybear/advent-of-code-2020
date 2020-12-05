pub fn calculate(s: &str) -> i32 {
    let mut checking = -1;
    let mut numbers: Vec<i32> = s.lines().map(|x| get_seat_id(x)).collect();
    numbers.sort();
    for num in &numbers {
        if checking > -1 && checking != *num {
            return checking;
        }
        checking = num + 1;
    }
    panic!("I can't find any available seats :(");
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
