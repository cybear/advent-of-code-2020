use super::lib;

pub fn solve(data: &Vec<lib::Adapter>) -> usize {
    println!("solving for {:?}", data);
    get_valid_permutations(&data, 0)
}

fn get_joltage_step(first: &lib::Adapter, second: &lib::Adapter) -> Option<usize> {
    let diff = second - first;
    match diff <= 3 {
        true => Some(diff),
        false => None,
    }
}

fn get_valid_permutations(data: &Vec<lib::Adapter>, start_index: usize) -> usize {
    let mut sum: usize = 0;
    let min = start_index + 1;
    let max = data.len();
    if max -1 == start_index {
        return 1;
    }
    let current_adapter = &data[start_index];
    for adapter_index in min..max {
        let next_adapter = &data[adapter_index];
        match get_joltage_step(current_adapter, next_adapter) {
            Some(n) => {
                sum += get_valid_permutations(&data, adapter_index)
            },
            None => {
                break
            }
        }
    }
    sum
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
    static TESTDATA2: &str = "28
33
18
42
31
14
46
20
48
47
24
23
49
45
19
38
39
11
1
32
25
35
8
17
7
9
4
2
34
10
3";
    #[test]
    fn test_solver() {
        assert_eq!(solve(&lib::parse_file(TESTDATA)), 8);
    }
    #[test]
    fn test_solver2() {
        assert_eq!(solve(&lib::parse_file(TESTDATA2)), 19208);
    }
}