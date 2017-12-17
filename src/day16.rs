use std::collections::HashMap;

#[derive(Copy, Clone, PartialEq, Debug)]
enum DanceMove {
    Spin(usize),
    Exchange(usize, usize),
    Partner(u8, u8),
}
use self::DanceMove::*;

fn parse_input(s: &str) -> Vec<DanceMove> {
    s.trim().split(',')
        .map(|m| match &m[..1] {
            "s" => Spin(m[1..].parse().unwrap()),
            "x" => { let d = m.find("/").unwrap();
                     Exchange(m[1..d].parse().unwrap(), m[d+1..].parse().unwrap())
                   }
            "p" => Partner(m.as_bytes()[1], m.as_bytes()[3]),
            _ => panic!("unknown dance move: {}", m),
        }).collect()
}

fn dance(num_dancers: usize, moves: &[DanceMove], reps: u32) -> Vec<u8> {
    let mut dancers: Vec<u8> = (b'a'..b'a'+num_dancers as u8).collect();
    let mut dance_memo = HashMap::new();

    for i in 0..reps {
        if let Some(k) = dance_memo.insert(dancers.clone(), i) {
            // We hit this state at k and then again at i.
            // The loop length is i-k.
            // We have reps-i iterations still to do, which
            // is (reps-i)/(i-k) full loops followed by
            // (reps-i)%(i-k) additional.
            // Which should be the same as the state we saw
            // at iteration k + (reps-i)%(i-k).
            let t = k + (reps - i) % (i - k);
            if let Some(x) = dance_memo.iter().find(|&(_,&j)| j == t) {
                return x.0.clone();
            }
        }

        for &m in moves.iter() {
            match m {
                Spin(x) => {
                        let y = num_dancers - x;
                        let mut tmp = dancers[y..].to_vec();
                        tmp.extend(&dancers[..y]);
                        dancers.copy_from_slice(&tmp);
                    }
                Exchange(p,q) => dancers.swap(p,q),
                Partner(a,b) => {
                        let a_pos = dancers.iter().position(|&x| x == a).unwrap();
                        let b_pos = dancers.iter().position(|&x| x == b).unwrap();
                        dancers.swap(a_pos,b_pos);
                    }
            }
        }
    }
    dancers
}

fn solve(input: &str) -> (String, String) {
    let moves = parse_input(input);

    let dancers = dance(16, &moves, 1);
    let part1 = String::from_utf8(dancers).unwrap();

    let dancers = dance(16, &moves, 1_000_000_000);
    let part2 = String::from_utf8(dancers).unwrap();

    (part1, part2)
}

pub fn run(input: &str) {
    let (part1, part2) = solve(input);
    println!("the arrangement after one round is {}", part1);
    println!("the arrangement after a billion is {}", part2);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parsing() {
        assert_eq!(parse_input("s1,x3/4,pe/b\n"),
                   vec![Spin(1), Exchange(3,4), Partner(b'e',b'b')]);
    }

    #[test]
    fn example1() {
        let moves = parse_input("s1,x3/4,pe/b");
        let dancers = dance(5, &moves, 1);
        assert_eq!(b"baedc", dancers.as_slice());
    }

    #[test]
    fn example2() {
        let moves = parse_input("s1,x3/4,pe/b");
        let dancers = dance(5, &moves, 2);
        assert_eq!(b"ceadb", dancers.as_slice());
    }

    #[test]
    fn real_input() {
        let input = include_str!("../inputs/day16.txt");
        let x = solve(&input);
        assert_eq!(include_str!("../outputs/day16.txt"),
                   format!("{:?}", x));
    }
}
