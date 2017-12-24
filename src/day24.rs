use std::collections::HashSet;

fn parse_input(s: &str) -> Vec<(u32,u32)> {
    s.lines().map(|line| {
            let mut p = line.split('/').map(|m| m.parse().unwrap());
            (p.next().unwrap(), p.next().unwrap())
        }).collect()
}

#[derive(Debug)]
struct State {
    connector: u32,
    components: HashSet<(u32,u32)>,
    strength: u32,
    length: u32,
}

impl State {
    fn extend(&self) -> Vec<State> {
        self.components.iter()
            .filter(|&&(a,b)| a == self.connector || b == self.connector)
            .map(|&(a,b)| {
                let mut new_comps = self.components.clone();
                new_comps.remove(&(a,b));
                State {
                    connector: if self.connector == a { b } else { a },
                    components: new_comps,
                    strength: self.strength + a + b,
                    length: self.length + 1,
                }
            }).collect()
    }
}

fn solve(input: &str) -> (u32, u32) {
    let components = parse_input(input);
    let components: HashSet<_> = components.into_iter().collect();

    let mut stack = vec![State { connector: 0, components, strength: 0, length: 0 }];
    let mut strongest = 0;
    let mut best = (0,0);
    while let Some(st) = stack.pop() {
        let next = st.extend();
        if next.len() == 0 {
            if st.strength > strongest { strongest = st.strength; }
            let q = (st.length, st.strength);
            if q > best { best = q; }
        } else {
            stack.extend(next);
        }
    }
    (strongest, best.1)
}

pub fn run(input: &str) {
    let (part1, part2) = solve(input);
    println!("the strongest bridge is {}", part1);
    println!("the strongest longest bridge is {}", part2);
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &'static str = "\
0/2
2/2
2/3
3/4
3/5
0/1
10/1
9/10
";

    #[test]
    fn parsing() {
        assert_eq!(parse_input(EXAMPLE),
                   vec![(0,2), (2,2), (2,3), (3,4),
                        (3,5), (0,1), (10,1), (9,10)]);
    }

    #[test]
    fn example1() {
        let (part1, part2) = solve(EXAMPLE);
        assert_eq!(31, part1);
        assert_eq!(19, part2);
    }

    #[test]
    fn real_input() {
        let input = include_str!("../inputs/day24.txt");
        let x = solve(&input);
        assert_eq!(include_str!("../outputs/day24.txt"),
                   format!("{:?}", x));
    }
}
