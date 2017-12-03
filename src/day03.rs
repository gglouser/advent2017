use std::collections::HashMap;

fn parse_input(s: &str) -> i32 {
    s.trim().parse().unwrap()
}

struct SpiralGen {
    pos: (i32,i32),
    dir: (i32,i32),
    side: i32,
    side_i: i32,
    side_c: i32,
}

impl SpiralGen {
    fn new() -> SpiralGen {
        SpiralGen {
            pos: (0,0),
            dir: (1,0),
            side: 1,
            side_i: 0,
            side_c: 0,
        }
    }
}

impl Iterator for SpiralGen {
    type Item = (i32,i32);
    fn next(&mut self) -> Option<Self::Item> {
        let p = self.pos;
        self.pos.0 += self.dir.0;
        self.pos.1 += self.dir.1;
        self.side_i += 1;
        if self.side_i >= self.side {
            self.side_i = 0;
            self.dir = (-self.dir.1, self.dir.0);
            self.side_c += 1;
            if self.side_c >= 2 {
                self.side_c = 0;
                self.side += 1;
            }
        }
        Some(p)
    }
}

fn sum_around(grid: &HashMap<(i32,i32),i32>, pos: (i32,i32)) -> i32 {
    let mut sum = 0;
    for i in pos.0-1..pos.0+2 {
        for j in pos.1-1..pos.1+2 {
            sum += *grid.get(&(i,j)).unwrap_or(&0);
        }
    }
    sum
}

fn find_first_gt(key: i32) -> i32 {
    let mut spiral = SpiralGen::new();
    let mut grid: HashMap<(i32,i32),i32> = HashMap::new();
    grid.insert((0,0), 1);
    spiral.next(); // discard (0,0)
    for pos in spiral {
        let x = sum_around(&grid, pos);
        if x > key {
            return x;
        }
        grid.insert(pos, x);
    }
    0
}

fn solve(input: &str) -> (i32,i32) {
    let key = parse_input(input);
    let pos = SpiralGen::new().nth(key as usize - 1).unwrap();
    let part1 = pos.0.abs() + pos.1.abs();
    let part2 = find_first_gt(key);
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
        assert_eq!(parse_input("3\n"), 3);
    }

    #[test]
    fn example1() {
        assert_eq!(0, solve("1").0);
        assert_eq!(3, solve("12").0);
        assert_eq!(2, solve("23").0);
        assert_eq!(31, solve("1024").0);
    }

    #[test]
    fn real_input() {
        let input = include_str!("../inputs/day03.txt");
        let x = solve(&input);
        assert_eq!(include_str!("../outputs/day03.txt"),
                   format!("{:?}", x));
    }
}
