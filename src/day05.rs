fn parse_input(s: &str) -> Vec<i32> {
    s.lines()
        .map(|m| m.parse().unwrap())
        .collect()
}

fn sim<F>(mut jumps: Vec<i32>, jump_fn: F) -> u32 where
    F: Fn(i32) -> i32
{
    let mut pc = 0;
    let mut steps = 0;
    while let Some(ptr) = jumps.get_mut(pc as usize) {
        let jump = *ptr;
        *ptr = jump_fn(jump);
        pc += jump;
        steps += 1;
    }
    steps
}

fn solve(input: &str) -> (u32,u32) {
    let jumps = parse_input(input);
    let part1 = sim(jumps.clone(), |j| j+1);
    let part2 = sim(jumps.clone(), |j| if j < 3 { j+1 } else { j-1 });
    (part1, part2)
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
        assert_eq!(parse_input("0\n3\n0\n1\n-3\n"),
                   vec![0, 3, 0, 1, -3]);
    }

    #[test]
    fn example1() {
        let (part1, part2) = solve("0\n3\n0\n1\n-3\n");
        assert_eq!(5, part1);
        assert_eq!(10, part2);
    }

    #[test]
    fn real_input() {
        let input = include_str!("../inputs/day05.txt");
        let x = solve(&input);
        assert_eq!(include_str!("../outputs/day05.txt"),
                   format!("{:?}", x));
    }
}
