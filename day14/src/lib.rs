use std::fmt;

#[derive(Debug)]
pub struct Instruction {
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

#[derive(Debug)]
pub struct Program {
    mask: String,
    instructions: Vec<Instruction>,
}

impl fmt::Display for Program {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let instructions_s: Vec<String> = self.instructions.iter().map(
            |instruction| format!(
                "Binary: {:036b}  Addr: {}   (dec: {})",
                instruction.value, instruction.address, instruction.value
            )
        ).collect();
        write!(
            f,
            "Mask:   {}\n{}",
            self.mask,
            instructions_s.join("\n"),
        )
    }
}
impl PartialEq for Program {
    fn eq(&self, other: &Self) -> bool {
        if self.mask.ne(&other.mask) {
            return false;
        }
        if self.instructions.len() != other.instructions.len() {
            return false;
        }
        for x in 0..self.instructions.len() {
            if self.instructions[x] != other.instructions[x] {
                return false;
            }
        }
        true
    }
}
impl Eq for Program {}

pub fn solve(data: Vec<Program>) -> usize {
    data.iter().for_each(|program| println!("{}", program));
    1
}

fn parse_instruction(s: &str) -> Instruction {
    let mut split = s.split("] = ");
    let address = split.next().unwrap()
        .replace("mem[", "")
        .replace("]", "")
        .parse().unwrap();
    let value = split.next().unwrap()
        .parse().unwrap();
    Instruction {
        address,
        value,
    }
}

pub fn parse_file(s: &str) -> Vec<Program> {
    println!("parsing file {}", s);
    s
        .split("\nmask = ") // Only split on each item after the first line
        .map(
            |program_s| {
                println!("Program {}", program_s);
                let mut lines = program_s.lines();
                let mask = lines.next().unwrap().replace("mask = ","").to_string(); // Clean the shit up
                let instructions = lines.map(parse_instruction).collect();
                Program {
                    mask,
                    instructions,
                }
            }
        ).collect::<Vec<Program>>()
}

fn apply_mask(mem: u64, mask: &str, value: u64) -> u64 {

    mem
}

#[cfg(test)]
mod tests {
    use super::*;

    static TESTDATA: &str = "mask = XXXXXXXXXXXXXXXXXXXXXXXXXXXXX1XXXX0X
mem[8] = 11
mem[7] = 101
mem[8] = 0";
    #[test]
    fn test_parse_file() {
        let notes = parse_file(TESTDATA);
        let expected = Program {
            mask: "XXXXXXXXXXXXXXXXXXXXXXXXXXXXX1XXXX0X".to_string(),
            instructions: vec![
                Instruction {address: 8, value: 11},
                Instruction {address: 7, value: 101},
                Instruction {address: 8, value: 0},
            ],
        };
        assert_eq!(notes[0], expected);
    }
    #[test]
    fn test_binary() {
        let instruction = Instruction {address: 8, value: 11};
        assert_eq!(format!("{:036b}", instruction.value), "000000000000000000000000000000001011");
    }
    // #[test]
    // fn test_apply_mask() {
    //     let notes = parse_file(TESTDATA);
    //     let result = apply_mask(mem: u64, mask: &str, value: u64) -> u64
    //     assert_eq!()
    // }
}
