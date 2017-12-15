fn parse_input(s: &str) -> (u32,u32) {
    let ys: Vec<_> = s.lines()
        .map(|line| {
            let xs: Vec<_> = line.split_whitespace().collect();
            xs[xs.len()-1].parse().unwrap()
        }).collect();
    (ys[0], ys[1])
}

#[derive(Clone)]
struct Generator {
    factor: u64,
    seed: u64,
}

impl Iterator for Generator {
    type Item = u32;
    fn next(&mut self) -> Option<Self::Item> {
        self.seed = (self.seed * self.factor) % 2147483647;
        Some(self.seed as u32)
    }
}

fn solve(input: &str) -> (usize, usize) {
    let (init_a, init_b) = parse_input(input);

    let a = Generator { factor: 16807, seed: init_a as u64 };
    let b = Generator { factor: 48271, seed: init_b as u64 };

    let part1 = a.clone()
        .zip(b.clone())
        .take(40_000_000)
        .filter(|&(a,b)| a & 0xffff == b & 0xffff)
        .count();

    let part2 = a.filter(|&x| x % 4 == 0)
        .zip(b.filter(|&x| x % 8 == 0))
        .take(5_000_000)
        .filter(|&(a,b)| a & 0xffff == b & 0xffff)
        .count();

    (part1, part2)
}

pub fn run(input: &str) {
    let (part1, part2) = solve(input);
    println!("the judge's first count is {}", part1);
    println!("the judge's second count is {}", part2);
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &'static str = "\
Generator A starts with 65
Generator B starts with 8921
";

    #[test]
    fn parsing() {
        assert_eq!(parse_input(EXAMPLE),
                   (65, 8921));
    }

    #[test]
    fn example1() {
        let (part1,part2) = solve(EXAMPLE);
        assert_eq!(588, part1);
        assert_eq!(309, part2);
    }

    #[test]
    fn real_input() {
        let input = include_str!("../inputs/day15.txt");
        let x = solve(&input);
        assert_eq!(include_str!("../outputs/day15.txt"),
                   format!("{:?}", x));
    }
}
