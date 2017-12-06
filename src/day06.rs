use std::collections::HashMap;

fn parse_input(s: &str) -> Vec<u32> {
    s.trim().split_whitespace()
        .map(|m| m.parse().unwrap())
        .collect()
}

fn find_most(banks: &[u32]) -> (usize, u32) {
    let (i,c) = banks.iter().enumerate()
        .max_by(|a,b| a.1.cmp(b.1).then(b.0.cmp(&a.0)))
        .unwrap();
    (i,*c)
}

fn reallocate(banks: &mut [u32]) {
    let (mut i, mut c) = find_most(banks);
    banks[i] = 0;
    while c > 0 {
        i = (i + 1) % banks.len();
        banks[i] += 1;
        c -= 1;
    }
}

fn solve(input: &str) -> (u32, u32) {
    let mut banks = parse_input(input);
    let mut states = HashMap::new();
    let mut count = 0;
    loop {
        if let Some(k) = states.insert(banks.clone(), count) {
            return (count, count - k);
        }
        reallocate(&mut banks);
        count += 1;
    }
}

pub fn run(input: &str) {
    let (part1, part2) = solve(input);
    println!("the first repeat is at {}", part1);
    println!("the loop size is {}", part2);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parsing() {
        assert_eq!(parse_input("0 1 10 100\n"),
                   vec![0, 1, 10, 100]);
    }

    #[test]
    fn example1() {
        let (end, loop_size) = solve("0 2 7 0");
        assert_eq!(5, end);
        assert_eq!(4, loop_size);
    }

    #[test]
    fn real_input() {
        let input = include_str!("../inputs/day06.txt");
        let x = solve(&input);
        assert_eq!(include_str!("../outputs/day06.txt"),
                   format!("{:?}", x));
    }
}
