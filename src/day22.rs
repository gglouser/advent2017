use std::collections::HashMap;

#[derive(Copy, Clone, PartialEq, Debug)]
enum NodeState {
    Clean,
    Weakened,
    Infected,
    Flagged,
}
use self::NodeState::*;

type Coord = (i32, i32);
type Grid = HashMap<Coord, NodeState>;

fn parse_input(s: &str) -> (Grid, Coord) {
    let mut lines = s.lines().peekable();
    let width = lines.peek().unwrap().len() as i32;
    let start = (width / 2, lines.count() as i32 / 2);
    let grid = s.lines().enumerate()
        .flat_map(|(row,line)|
            line.bytes().enumerate()
                .map(move |(col,b)|
                    ((col as i32, row as i32),
                     if b == b'#' { Infected } else { Clean })))
        .collect();
    (grid, start)
}

fn change_heading(heading: Coord, st: NodeState) -> Coord {
    match st {
        Clean => (heading.1, -heading.0), // left
        Weakened => heading,
        Infected => (-heading.1, heading.0), // right
        Flagged => (-heading.0, -heading.1), // reverse
    }
}

trait Virus {
    fn change_state(st: NodeState) -> NodeState;
}

struct Virus1();
impl Virus for Virus1 {
    fn change_state(st: NodeState) -> NodeState {
        match st {
            Clean => Infected,
            _     => Clean,
        }
    }
}

struct Virus2();
impl Virus for Virus2 {
    fn change_state(st: NodeState) -> NodeState {
        match st {
            Clean    => Weakened,
            Weakened => Infected,
            Infected => Flagged,
            Flagged  => Clean,
        }
    }
}

fn simulate<T: Virus>(mut grid: Grid, start: Coord, bursts: u32) -> u32 {
    let mut pos = start;
    let mut heading = (0, -1);
    let mut infections = 0;
    for _ in 0..bursts {
        let e = grid.entry(pos).or_insert(Clean);
        heading = change_heading(heading, *e);
        let new_st = T::change_state(*e);
        *e = new_st;
        if new_st == Infected { infections += 1; }
        pos = (pos.0 + heading.0, pos.1 + heading.1);
    }
    infections
}

fn solve(input: &str) -> (u32, u32) {
    let (grid, start) = parse_input(input);
    let part1 = simulate::<Virus1>(grid.clone(), start, 10_000);
    let part2 = simulate::<Virus2>(grid, start, 10_000_000);
    (part1, part2)
}

pub fn run(input: &str) {
    let (part1, part2) = solve(input);
    println!("the number of infections is {}", part1);
    println!("the number of infections for mutated virus is {}", part2);
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &'static str = "..#\n\
                                   #..\n\
                                   ...\n";

    #[test]
    fn parsing() {
        let (grid, start) = parse_input(EXAMPLE);
        assert_eq!((1,1), start);
        assert_eq!(Clean, grid[&(0,0)]);
        assert_eq!(Clean, grid[&(1,0)]);
        assert_eq!(Infected, grid[&(2,0)]);
        assert_eq!(Infected, grid[&(0,1)]);
        assert_eq!(Clean, grid[&(1,1)]);
        assert_eq!(Clean, grid[&(2,1)]);
        assert_eq!(Clean, grid[&(0,2)]);
        assert_eq!(Clean, grid[&(1,2)]);
        assert_eq!(Clean, grid[&(2,2)]);
    }

    #[test]
    fn example1() {
        let (grid, start) = parse_input(EXAMPLE);
        let infected = simulate::<Virus1>(grid, start, 70);
        assert_eq!(41, infected);
    }

    #[test]
    fn example2() {
        let (grid, start) = parse_input(EXAMPLE);
        let infected = simulate::<Virus2>(grid, start, 100);
        assert_eq!(26, infected);
    }

    #[test]
    #[ignore]
    fn example3() {
        let (grid, start) = parse_input(EXAMPLE);
        let infected = simulate::<Virus2>(grid, start, 10_000_000);
        assert_eq!(2511944, infected);
    }

    #[test]
    #[ignore]
    fn real_input() {
        let input = include_str!("../inputs/day22.txt");
        let x = solve(&input);
        assert_eq!(include_str!("../outputs/day22.txt"),
                   format!("{:?}", x));
    }
}
