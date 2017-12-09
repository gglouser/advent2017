fn solve(stream: &str) -> (u32, u32) {
    let mut depth = 0;
    let mut in_garbage = false;
    let mut skip_next = false;
    let mut chars = stream.chars();
    let mut score = 0;
    let mut garbage_count = 0;

    chars.next().unwrap(); // initial {
    depth += 1;
    while depth > 0 {
        let c = chars.next().unwrap();
        if !in_garbage {
            match c {
                '{' => depth += 1,
                '}' => { score += depth; depth -= 1; },
                '<' => in_garbage = true,
                ',' => (),
                _ => panic!("unexpected char: '{}'", c)
            }
        } else {
            if skip_next {
                skip_next = false;
            } else {
                match c {
                    '>' => in_garbage = false,
                    '!' => skip_next = true,
                    _ => garbage_count += 1,
                }
            }
        }
    }

    (score, garbage_count)
}

pub fn run(input: &str) {
    let (part1, part2) = solve(input);
    println!("the solution to part 1 is {}", part1);
    println!("the solution to part 2 is {}", part2);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example1() {
        assert_eq!(solve("{}").0, 1);
        assert_eq!(solve("{{{}}}").0, 6);
        assert_eq!(solve("{{},{}}").0, 5);
        assert_eq!(solve("{{{},{},{{}}}}").0, 16);
        assert_eq!(solve("{<a>,<a>,<a>,<a>}").0, 1);
        assert_eq!(solve("{{<ab>},{<ab>},{<ab>},{<ab>}}").0, 9);
        assert_eq!(solve("{{<!!>},{<!!>},{<!!>},{<!!>}}").0, 9);
        assert_eq!(solve("{{<a!>},{<a!>},{<a!>},{<ab>}}").0, 3);
    }

    #[test]
    fn example2() {
        assert_eq!(solve("{<>}").1, 0);
        assert_eq!(solve("{<random characters>}").1, 17);
        assert_eq!(solve("{<<<<>}").1, 3);
        assert_eq!(solve("{<{!>}>}").1, 2);
        assert_eq!(solve("{<!!>}").1, 0);
        assert_eq!(solve("{<!!!>>}").1, 0);
        assert_eq!(solve("{<{o\"i!a,<{i<a>}").1, 10);
    }

    #[test]
    fn real_input() {
        let input = include_str!("../inputs/day09.txt");
        let x = solve(&input);
        assert_eq!(include_str!("../outputs/day09.txt"),
                   format!("{:?}", x));
    }
}
