use super::lib;

pub fn solve(data: &Vec<lib::Adapter>) -> usize {
    println!("solving for {:?}", data);
    let iter = data.windows(2);
    let steps = iter.map(|x| {
        get_joltage_step(&x[0], &x[1])
    }).collect();
    let chunks = split_into_chunks(&steps);
    chunks.iter().map(
            |x| solve_chunk(x)
    ).fold(0, |a, b| a+b)
}

fn solve_chunk(data: &Vec<usize>) -> usize {
    println!("solving for chunk {:?}", data);
    get_valid_permutations(&data, 0)
}

fn split_into_chunks(arr: &Vec<usize>) -> Vec<Vec<usize>> {
    // :shrug:
    let joltage_string: String = arr
        .iter()
        .map(|x| x.to_string())   // 3 => "3"
        .collect::<Vec<String>>() // An array of strings
        .join("");                // join into one big string.

    println!("Joltage string {:?}", joltage_string);

    let chunks: Vec<Vec<usize>> = joltage_string
        .split(|x| x == '3') // split on each "3"
        .map(|x| str_to_joltage_numbers(x))
        .collect();
    
        chunks
}

fn str_to_joltage_numbers(s: &str) -> Vec<usize> {
    s.chars().map(
        |s: char| s.to_string().parse().unwrap()
    ).collect()
}

fn get_joltage_step(first: &lib::Adapter, second: &lib::Adapter) -> usize {
    println!("comparing {} to {} = {}", first, second, second - first);
    second - first
}

fn get_valid_permutations(data: &Vec<usize>, start_index: usize) -> usize {
    let mut sum: usize = 0;
    let min = start_index + 1;
    let max = data.len();
    if max -1 == start_index {
        return 1;
    }
    let current_adapter = &data[start_index];
    for adapter_index in min..max {
        let next_adapter = &data[adapter_index];
        println!("Comparing {} {}", next_adapter, current_adapter);
        if next_adapter - current_adapter < 3 {
            sum += get_valid_permutations(&data, adapter_index)
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
    fn test_partitioning() {
        assert_eq!(solve(&lib::parse_file(TESTDATA)), 1);
    }

    // fn test_solver() {
    //     assert_eq!(solve(&lib::parse_file(TESTDATA)), 8);
    // }
    // #[test]
    // fn test_solver2() {
    //     assert_eq!(solve(&lib::parse_file(TESTDATA2)), 19208);
    // }
}