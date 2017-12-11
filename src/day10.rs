use std::fmt::Write;

fn twist(xs: &mut [u32], k: usize) {
    if k == 0 { return; }
    let section = &mut xs[..k];
    section.reverse();
}

fn rotate(xs: &mut [u32], k: usize) {
    if k == 0 { return; }
    let mut new_xs = xs[k..].to_vec();
    new_xs.extend_from_slice(&xs[..k]);
    xs.copy_from_slice(&new_xs);
}

fn apply_lengths(size: usize, lengths: &[usize], rounds: u32) -> Vec<u32> {
    let mut elems = (0..size as u32).collect::<Vec<_>>();
    let mut skip = 0;
    let mut offset = 0;

    for _ in 0..rounds {
        for &l in lengths {
            twist(&mut elems, l);
            rotate(&mut elems, (l + skip) % size);
            offset = (offset + l + skip) % size;
            skip += 1;
        }
    }
    rotate(&mut elems, size - offset);
    elems
}

const SUGAR: &[usize] = &[17, 31, 73, 47, 23];

fn prep_key(keystr: &str) -> Vec<usize> {
    let mut key = Vec::with_capacity(keystr.len() + SUGAR.len());
    keystr.bytes().for_each(|b| key.push(b as usize));
    key.extend(SUGAR);
    key
}

fn knot_hash(text: &str) -> String {
    let part2_key = prep_key(text);
    let sparse = apply_lengths(256, &part2_key, 64);
    let dense: Vec<_> = sparse.chunks(16)
        .map(|ch| ch.iter().fold(0, |acc, &x| acc ^ x))
        .collect();

    let mut hash = String::new();
    dense.iter().for_each(|b| write!(hash, "{:02x}", b).unwrap());
    hash
}

fn solve(input: &str) -> (u32, String) {
    let part1_key: Vec<usize> = input.trim().split(",")
        .map(|m| m.parse().unwrap())
        .collect();
    let xs = apply_lengths(256, &part1_key, 1);
    let part1 = xs[0] * xs[1];

    let part2 = knot_hash(input.trim());

    (part1, part2)
}

pub fn run(input: &str) {
    let (part1, part2) = solve(input);
    println!("the single round test value is {}", part1);
    println!("the knot hash is {}", part2);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example1() {
        let xs = apply_lengths(5, &[3,4,1,5], 1);
        let part1 = xs[0] * xs[1];
        assert_eq!(12, part1);
    }

    #[test]
    fn keyprep() {
        assert_eq!(prep_key("1,2,3"), vec![49,44,50,44,51,17,31,73,47,23]);
    }

    #[test]
    fn example2() {
        assert_eq!(knot_hash(""), "a2582a3a0e66e6e86e3812dcb672a272");
        assert_eq!(knot_hash("AoC 2017"), "33efeb34ea91902bb2f59c9920caa6cd");
        assert_eq!(knot_hash("1,2,3"), "3efbe78a8d82f29979031a4aa0b16a9d");
        assert_eq!(knot_hash("1,2,4"), "63960835bcdc130f0b66d7ff4f6a5a8e");
    }

    #[test]
    fn real_input() {
        let input = include_str!("../inputs/day10.txt");
        let x = solve(&input);
        assert_eq!(include_str!("../outputs/day10.txt"),
                   format!("{:?}", x));
    }
}
