use std::iter;

#[derive(Copy, Clone, PartialEq, Debug)]
enum Move { R, L }

type Action = (u8, Move, usize);
type State = Vec<Action>;

fn parse_input(s: &str) -> (u32, Vec<State>) {
    let mut lines = s.lines();
    assert_eq!(lines.next().unwrap(), "Begin in state A.");

    let steps = lines.next().unwrap()
        .trim_left_matches("Perform a diagnostic checksum after ")
        .trim_right_matches(" steps.")
        .parse().unwrap();

    let mut states = Vec::new();
    while let Some(_) = lines.next() {
        let _state_name = lines.next().unwrap()
            .trim_left_matches("In state ")
            .trim_right_matches(":");
        assert_eq!(_state_name.len(), 1);
        assert_eq!((_state_name.as_bytes()[0] - b'A') as usize, states.len());
        assert_eq!(lines.next().unwrap(), "  If the current value is 0:");
        let a0 = parse_action(&mut lines);
        assert_eq!(lines.next().unwrap(), "  If the current value is 1:");
        let a1 = parse_action(&mut lines);
        states.push(vec![a0, a1]);
    }

    (steps, states)
}

fn parse_action<'a, I: Iterator<Item = &'a str>>(lines: &mut I) -> Action {
    let w = lines.next().unwrap()
        .trim_left_matches("    - Write the value ")
        .trim_right_matches(".")
        .parse().unwrap();

    let m = match lines.next().unwrap()
        .trim_left_matches("    - Move one slot to the ")
        {
            "right." => Move::R,
            "left."  => Move::L,
            x => panic!("unknown direction: '{}'", x),
        };

    let s_name = lines.next().unwrap()
        .trim_left_matches("    - Continue with state ")
        .trim_right_matches(".");
    assert_eq!(s_name.len(), 1);
    let s = s_name.as_bytes()[0] - b'A';

    (w, m, s as usize)
}

struct Tape {
    cursor: u8,
    left: Vec<u8>,
    right: Vec<u8>,
}

impl Tape {
    fn new() -> Tape {
        Tape { cursor: 0, left: vec![], right: vec![] }
    }

    #[inline]
    fn read(&self) -> u8 { self.cursor }

    #[inline]
    fn write(&mut self, x: u8) { self.cursor = x; }

    #[inline]
    fn move_left(&mut self) {
        self.right.push(self.cursor);
        self.cursor = self.left.pop().unwrap_or(0);
    }

    #[inline]
    fn move_right(&mut self) {
        self.left.push(self.cursor);
        self.cursor = self.right.pop().unwrap_or(0);
    }

    fn count_ones(&self) -> usize {
        self.left.iter()
            .chain(iter::once(&self.cursor))
            .chain(self.right.iter().rev())
            .filter(|&&x| x == 1)
            .count()
    }
}

fn diagnostic(prog: Vec<State>, steps: u32) -> usize {
    let mut tape = Tape::new();
    let mut st = &prog[0];
    for _ in 0..steps {
        let (w,mv,s) = st[tape.read() as usize];
        tape.write(w);
        match mv {
            Move::R => tape.move_right(),
            Move::L => tape.move_left(),
        }
        st = &prog[s];
    }
    tape.count_ones()
}

fn solve(input: &str) -> usize {
    let (steps, prog) = parse_input(input);
    let checksum = diagnostic(prog, steps);
    checksum
}

pub fn run(input: &str) {
    let part1 = solve(input);
    println!("the diagnostic checksum is {}", part1);
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &'static str = "\
Begin in state A.
Perform a diagnostic checksum after 6 steps.

In state A:
  If the current value is 0:
    - Write the value 1.
    - Move one slot to the right.
    - Continue with state B.
  If the current value is 1:
    - Write the value 0.
    - Move one slot to the left.
    - Continue with state B.

In state B:
  If the current value is 0:
    - Write the value 1.
    - Move one slot to the left.
    - Continue with state A.
  If the current value is 1:
    - Write the value 1.
    - Move one slot to the right.
    - Continue with state A.
";

    #[test]
    fn parsing() {
        assert_eq!(parse_input(EXAMPLE),
                   (6, vec![vec![(1,Move::R,1), (0,Move::L,1)],
                            vec![(1,Move::L,0), (1,Move::R,0)]]));
    }

    #[test]
    fn example1() {
        let part1 = solve(EXAMPLE);
        assert_eq!(3, part1);
    }

    #[test]
    fn real_input() {
        let input = include_str!("../inputs/day25.txt");
        let x = solve(&input);
        assert_eq!(include_str!("../outputs/day25.txt"),
                   format!("{:?}", x));
    }
}
