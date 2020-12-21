use std::fmt;

#[derive(Debug)]
pub struct Instruction {
    mask: Vec<char>,
    address: u64,
    value: u64,
}

impl fmt::Display for Instruction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Address: {}   Binary value: {:036b}   Value: {}",
            self.address,
            self.value,
            self.value,
        )
    }
}

impl PartialEq for Instruction {
    fn eq(&self, other: &Self) -> bool {
        self.address == other.address
        && self.value == other.value
    }
}

pub fn solve(data: Vec<Instruction>) -> u64 {
    data.iter().map(|x| x.value).sum()
}

pub fn parse_file(s: &str) -> Vec<Instruction> {
    let mut instructions = vec![];
    let mut mask = vec![];
    println!("parsing file {}", s);
    s
        .lines()
        .for_each(
            |line| {
                let prefix = &line[..4];
                match prefix {
                    "mask" => {
                        mask = line[8..].to_string().chars().collect();
                    },
                    "mem[" => {
                        let mut split = line.split("] = ");
                        let address = split.next().unwrap()
                            .replace("mem[", "")
                            .replace("]", "")
                            .parse().unwrap();
                        let value = split.next().unwrap()
                            .parse().unwrap();
                        let mask = mask.clone();
                        println!("Got something for ya, {} {}", address, value);
                        instructions.push(Instruction {
                            mask,
                            address,
                            value,
                        });
                    },
                    _ => panic!("This bad yo: {}", prefix)
                }
            }
        );
    instructions
}

fn apply_mask(instruction: Instruction) -> u64 {
    let value = format!("{:036b}", instruction.value);
    let two: u64 = 2;
    let mut number: u64 = 0;
    let ones = instruction.mask.iter().map(|c| match c {
        'X' | '1' => 0,
        '0' => 1,
        _ => panic!("No such character: {}", c),
    });
    let zeroes = instruction.mask.iter().map(|c| match c {
        'X' | '0' => 0,
        '1' => 1,
        _ => panic!("No such character: {}", c),
    });
    
    for (index, c) in instruction.mask.iter().enumerate() {
        let index64 = index as u32;
        match c {
            'X' => (),
            '0' => number += two.pow(index64),
            _ => panic!("No such char: {}", c),
        }
    }
    number
}

#[cfg(test)]
mod tests {
    use super::*;

    static TESTDATA: &str = "mask = XXXXXXXXXXXXXXXXXXXXXXXXXXXXX1XXXX0X
mem[8] = 11
mem[7] = 101
mem[8] = 0";
}
