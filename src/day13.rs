fn parse_input(s: &str) -> Vec<(u32,u32)> {
    s.lines().map(|line| {
            let mut s = line.split(": ")
                .map(|m| m.parse().unwrap());
            let a = s.next().unwrap();
            let b = s.next().unwrap();
            (a,b)
        }).collect()
}

fn solve(input: &str) -> (u32, u32) {
    let layers = parse_input(input);
    
    let severity = layers.iter()
        .filter(|&&(depth, range)| depth % (2*range - 2) == 0)
        .map(|&(depth, range)| depth * range)
        .sum();
    
    let delay = (0..)
        .filter(|d|
            layers.iter().all(|&(depth, range)|
                (d + depth) % (2*(range - 1)) != 0
            )
        ).next().unwrap();

    (severity, delay)
}

pub fn run(input: &str) {
    let (part1, part2) = solve(input);
    println!("the severity is {}", part1);
    println!("the first safe delay is {}", part2);
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &'static str = "\
0: 3
1: 2
4: 4
6: 4
";

    #[test]
    fn parsing() {
        assert_eq!(parse_input(EXAMPLE),
                   vec![(0,3), (1,2), (4,4), (6,4)]);
    }

    #[test]
    fn example1() {
        let (part1,part2) = solve(EXAMPLE);
        assert_eq!(24, part1);
        assert_eq!(10, part2);
    }

    #[test]
    fn real_input() {
        let input = include_str!("../inputs/day13.txt");
        let x = solve(&input);
        assert_eq!(include_str!("../outputs/day13.txt"),
                   format!("{:?}", x));
    }
}
