use std::cmp::max;

fn parse_input(s: &str) -> Vec<&str> {
    s.trim().split(',').collect()
}

fn hexdist(x: i32, y: i32) -> i32 {
    if x > 0 && y > 0 || x < 0 && y < 0 {
        x.abs() + y.abs()
    } else {
        max(x.abs(), y.abs())
    }
}

fn solve(input: &str) -> (i32, i32) {
    let steps = parse_input(input);
    let mut x = 0;
    let mut y = 0;
    let mut farthest = 0;
    for step in steps {
        match step {
            "n" => y += 1,
            "ne" => x += 1,
            "se" => { x += 1; y -= 1; },
            "s" => y -= 1,
            "sw" => x -= 1,
            "nw" => { x -= 1; y += 1; },
            _ => panic!("unknown direction: {}", step)
        }
        farthest = max(farthest, hexdist(x, y));
    }
    (hexdist(x, y), farthest)
}

pub fn run(input: &str) {
    let (part1, part2) = solve(input);
    println!("fewest steps to reach the child: {}", part1);
    println!("the furthest he ever got was {}", part2);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parsing() {
        assert_eq!(parse_input("n,ne,se,s,sw,nw\n"),
                   vec!["n","ne","se","s","sw","nw"]);
    }

    #[test]
    // #[ignore]
    fn example1() {
        assert_eq!(3, solve("ne,ne,ne").0);
        assert_eq!(0, solve("ne,ne,sw,sw").0);
        assert_eq!(2, solve("ne,ne,s,s").0);
        assert_eq!(3, solve("se,sw,se,sw,sw").0);
    }

    #[test]
    fn real_input() {
        let input = include_str!("../inputs/day11.txt");
        let x = solve(&input);
        assert_eq!(include_str!("../outputs/day11.txt"),
                   format!("{:?}", x));
    }
}
