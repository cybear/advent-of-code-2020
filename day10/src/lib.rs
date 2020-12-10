use std::ops::Sub;
use std::fmt;

// Debug: Tests want this
// PartialEq, Eq, PartialOrd, Ord: Allow sorting
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord,)]
pub struct Adapter {
    joltage: usize,
}

// Implement subtraction for this type
impl Sub for &Adapter {
    type Output = usize;
    fn sub(self, other: Self) -> Self::Output {
        self.joltage - other.joltage
    }
}

// Allow println for this type
impl fmt::Display for Adapter {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.joltage)
    }
}

pub fn parse_file(s: &str) -> Vec<Adapter> {
    let lines = s.lines();
    let parsed_lines = lines.map(|l| parse_line(l));
    let mut vec: Vec<Adapter> = parsed_lines.collect();
    vec.sort();
    // Wall plug hack
    vec.insert(0, parse_line("0"));
    // Laptop hack
    let last = &vec[vec.len() - 1];
    let joltage = last.joltage + 3;
    vec.push(parse_line(&joltage.to_string()));
    vec
}

pub fn parse_line(l: &str) -> Adapter {
    Adapter {
        joltage: l.parse().unwrap()
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    static TESTDATA: &str = "16
10
15
5
1
11
7
19
6
12
4";

    #[test]
    fn test_parser() {
        let expected: Vec<Adapter> = "0,1,4,5,6,7,10,11,12,15,16,19,22".split(",").map(|x|parse_line(x)).collect();
        assert_eq!(parse_file(TESTDATA), expected);
    }
}
