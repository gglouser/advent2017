struct Map<'a>(Vec<&'a [u8]>);

impl<'a> Map<'a> {
    fn new(s: &str) -> Map {
        Map(s.lines()
            .map(|line| line.as_bytes())
            .collect())
    }

    fn start(&self) -> (i32, i32) {
        (0, self.0[0].iter().position(|&b| b == b'|').unwrap() as i32)
    }

    fn get(&self, pos: (i32, i32)) -> u8 {
        self.0[pos.0 as usize][pos.1 as usize]
    }
}

fn solve(input: &str) -> (String, u32) {
    let map = Map::new(input);
    let mut pos = map.start();
    let mut heading = (1,0);
    let mut letters: Vec<u8> = Vec::new();
    let mut steps = 0;
    loop {
        pos.0 += heading.0;
        pos.1 += heading.1;
        steps += 1;
        let here = map.get(pos);
        if here >= b'A' && here <= b'Z' {
            letters.push(here);
        } else if here == b'+' {
            let left = (-heading.1, heading.0);
            heading = if map.get((pos.0 + left.0, pos.1 + left.1)) != b' ' {
                left
            } else {
                (heading.1, -heading.0)
            };
        } else if here == b' ' {
            break;
        }
    }

    (String::from_utf8(letters).unwrap(), steps)
}

pub fn run(input: &str) {
    let (part1, part2) = solve(input);
    println!("the letters encountered are {}", part1);
    println!("the number of steps was {}", part2);
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::str::from_utf8;

    const EXAMPLE: &[&[u8]] = &[
        b"     |          ",
        b"     |  +--+    ",
        b"     A  |  C    ",
        b" F---|----E|--+ ",
        b"     |  |  |  D ",
        b"     +B-+  +--+ ",
        b"                ",
        ];

    #[test]
    fn start() {
        let map = Map(EXAMPLE.to_vec());
        assert_eq!((0,5), map.start());
    }

    #[test]
    fn example1() {
        let example = EXAMPLE.join(&b'\n');
        let example_str = from_utf8(&example).unwrap();
        let (part1, part2) = solve(example_str);
        assert_eq!("ABCDEF", part1);
        assert_eq!(38, part2);
    }

    #[test]
    fn real_input() {
        let input = include_str!("../inputs/day19.txt");
        let x = solve(&input);
        assert_eq!(include_str!("../outputs/day19.txt"),
                   format!("{:?}", x));
    }
}
