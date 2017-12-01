fn parse_input(s: &str) -> Vec<u32> {
    s.trim().chars()
        .map(|c| c.to_digit(10).unwrap())
        .collect()
}

fn sum_match(input: &[u32], offset: usize) -> u32 {
    input.iter().zip(input.iter().cycle().skip(offset))
        .filter(|&(a,b)| a == b)
        .map(|p| p.0)
        .sum()
}

fn solve(input: &str) -> (u32, u32) {
    let input = parse_input(input);
    let part1 = sum_match(&input, 1);
    let part2 = sum_match(&input, input.len()/2);
    (part1, part2)
}

pub fn run(input: &str) {
    let (part1, part2) = solve(input);
    println!("The first anti-captcha is {}", part1);
    println!("The second anti-captcha is {}", part2);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parsing() {
        assert_eq!(parse_input("0123456789\n"),
                   vec![0,1,2,3,4,5,6,7,8,9]);
    }

    #[test]
    fn example1() {
        assert_eq!(3, solve("1122").0);
        assert_eq!(4, solve("1111").0);
        assert_eq!(0, solve("1234").0);
        assert_eq!(9, solve("91212129").0);
    }

    #[test]
    fn example2() {
        assert_eq!(6, solve("1212").1);
        assert_eq!(0, solve("1221").1);
        assert_eq!(4, solve("123425").1);
        assert_eq!(12, solve("123123").1);
        assert_eq!(4, solve("12131415").1);
    }

    #[test]
    fn real_input() {
        let input = include_str!("../inputs/day01.txt");
        let x = solve(&input);
        assert_eq!(include_str!("../outputs/day01.txt"),
                   format!("{:?}", x));
    }
}
