use std::fmt;
pub struct Bus {
    pub number: usize,
    pub offset: usize,
}

#[derive(Debug)]
pub struct Notes {
    pub earliest_depart: usize,
    pub buses: Vec<usize>,
}
impl PartialEq for Notes {
    fn eq(&self, other: &Self) -> bool {
        if self.buses.len() != other.buses.len() {
            return false;
        }
        if  self.earliest_depart != other.earliest_depart {
            return false;
        }
        for x in 0..self.buses.len() {
            if self.buses[x] != other.buses[x] {
                return false;
            }
        }
        true
    }
}
impl Eq for Notes {}
impl fmt::Display for Notes {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let layout_arr: String = self.buses
            .iter().map(
                |bus| format!("bus {} ({})", bus, self.earliest_depart % bus),
            ).collect::<Vec<String>>().join(" ");
        write!(
            f,
            "{}",
            layout_arr)
    }
}

pub fn solve(data: Notes) -> usize {
    let mut deps = data.buses.iter().map(
        |bus| Bus {
            number: *bus,
            offset: bus - data.earliest_depart % bus,
        }
    ).collect::<Vec<Bus>>();
    deps.sort_by_key(|bus| bus.offset);
    let imminent_departure = &deps[0];
    imminent_departure.number * imminent_departure.offset
}

impl Notes {
    pub fn new(s: &str) -> Notes {
        println!("parsing file {}", s);
        let mut split = s.split("\n");
        let earliest_depart: usize = split.next().unwrap().parse().unwrap();
        let buses: Vec<usize> = split.next().unwrap().split(",")
        .filter(|x| *x != "x")
        .map(
            |n| {
                println!("n: {}", n);
                n.parse().unwrap()
            }
        ).collect();
        Notes {
            earliest_depart,
            buses,
        }    
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    static TESTDATA: &str = "939
7,13,x,x,59,x,31,19";
    #[test]
    fn test_parse_file() {
        let notes = Notes::new(TESTDATA);
        let expected = Notes {
            earliest_depart: 939,
            buses: vec![7,13,59,31,19],
        };
        assert_eq!(notes, expected);
    }
    #[test]
    fn test_solver() {
        let notes = Notes::new(TESTDATA);
        assert_eq!(solve(notes), 295);
    }
}
