use std::collections::HashMap;
use std::num::ParseIntError;
use std::ops::AddAssign;
use std::str::FromStr;

#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
struct Coord(i32, i32, i32);

impl Coord {
    fn abs(&self) -> i32 {
        self.0.abs() + self.1.abs() + self.2.abs()
    }
}

impl AddAssign for Coord {
    fn add_assign(&mut self, other: Coord) {
        self.0 += other.0;
        self.1 += other.1;
        self.2 += other.2;
    }
}

impl FromStr for Coord {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut parts = s[3..s.len()-1].split(',');
        let x = parts.next().unwrap().parse()?;
        let y = parts.next().unwrap().parse()?;
        let z = parts.next().unwrap().parse()?;
        Ok(Coord(x, y, z))
    }
}

#[derive(Clone, PartialEq, Debug)]
struct Particle {
    pos: Coord,
    vel: Coord,
    acc: Coord,
}

impl Particle {
    fn step(&mut self) {
        self.vel += self.acc;
        self.pos += self.vel;
    }
}

impl FromStr for Particle {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut parts = s.split(", ");
        let pos = parts.next().unwrap().parse()?;
        let vel = parts.next().unwrap().parse()?;
        let acc = parts.next().unwrap().parse()?;
        Ok(Particle { pos, vel, acc })
    }
}

fn parse_input(s: &str) -> Vec<Particle> {
    s.lines()
        .map(|line| line.parse().unwrap())
        .collect()
}

fn find_slowpoke(particles: &[Particle]) -> usize {
    particles.iter().enumerate()
        .min_by_key(|&(_,p)| (p.acc.abs(), p.vel.abs()))
        .unwrap().0
}

fn simulate(particles: &[Particle], iters: u32) -> Vec<Particle> {
    let mut ps = particles.to_vec();
    let mut pos_counts: HashMap<Coord, u32> = HashMap::new();
    for _ in 0..iters {
        pos_counts.clear();
        for p in ps.iter_mut() {
            p.step();
            let e = pos_counts.entry(p.pos).or_insert(0);
            *e += 1;
        }
        ps.retain(|p| pos_counts[&p.pos] == 1);
    }
    ps
}

fn solve(input: &str) -> (usize, usize) {
    let particles = parse_input(input);
    let part1 = find_slowpoke(&particles);

    // 100 cycles seems to be enough. Let's do 1000
    // println!("before {}", particles.len());
    let future = simulate(&particles, 1000);
    // println!("after {}", future.len());

    (part1, future.len())
}

pub fn run(input: &str) {
    let (part1, part2) = solve(input);
    println!("the solution to part 1 is {}", part1);
    println!("the solution to part 2 is {}", part2);
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &'static str = "\
p=<3,0,0>, v=<2,0,0>, a=<-1,0,0>
p=<4,0,0>, v=<0,0,0>, a=<-2,0,0>
";

    #[test]
    fn parsing() {
        assert_eq!(parse_input(EXAMPLE),
            vec![
                Particle{pos:Coord(3,0,0), vel:Coord(2,0,0), acc:Coord(-1,0,0)},
                Particle{pos:Coord(4,0,0), vel:Coord(0,0,0), acc:Coord(-2,0,0)},
            ]);
    }

    #[test]
    fn example1() {
        let (part1, part2) = solve(EXAMPLE);
        assert_eq!(0, part1);
        assert_eq!(2, part2);
    }

    #[test]
    fn real_input() {
        let input = include_str!("../inputs/day20.txt");
        let x = solve(&input);
        assert_eq!(include_str!("../outputs/day20.txt"),
                   format!("{:?}", x));
    }
}
