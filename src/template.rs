fn parse_input(s: &str) -> Vec<u32> {
    s.trim().split_whitespace()
        .map(|m| m.parse().unwrap())
        .collect()
}

fn solve(input: &str) -> (u32,u32) {
    let _input = parse_input(input);
    (0,0)
}

pub fn run(input: &str) {
    let (part1, part2) = solve(input);
    println!("the solution to part 1 is {:?}", part1);
    println!("the solution to part 2 is {:?}", part2);
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
        assert_eq!(0, solve("1 2 3").0);
    }

    #[test]
    fn example2() {
        assert_eq!(0, solve("1 2 3").1);
    }

    #[test]
    fn real_input() {
        let input = include_str!("../inputs/dayXX.txt");
        let x = solve(&input);
        assert_eq!(include_str!("../outputs/dayXX.txt"),
                   format!("{:?}", x));
    }
}
