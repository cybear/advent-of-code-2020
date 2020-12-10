use super::lib;

pub fn solve(data: Vec<lib::Adapter>) -> usize {
    println!("solving for {:?}", data);
    // The last plug actually can perform 3j higher
    let mut steps_count = [0, 0, 0, 1];
    let iter = data.windows(2);

    // wall hack
    let wall: lib::Adapter = lib::parse_line("0");
    let first_plug = &data[0];
    let first_diff = get_joltage_step(&wall, first_plug);
    steps_count[first_diff] += 1;
    println!("steps_count yo {:?}", steps_count);
    // end of wall hack

    let steps = iter.map(|x| {
        get_joltage_step(&x[0], &x[1])
    });
    steps.for_each(|step| steps_count[step] += 1);
    println!("steps_count yo {:?}", steps_count);

    steps_count[1] * steps_count[3]
}

fn get_joltage_step(first: &lib::Adapter, second: &lib::Adapter) -> usize {
    println!("comparing {} to {} = {}", first, second, second - first);
    second - first
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
        assert_eq!(solve(lib::parse_file(TESTDATA)), 7 * 5);
    }
    #[test]
    fn test_solver2() {
        assert_eq!(solve(lib::parse_file(TESTDATA2)), 22 * 10);
    }
}