use std::fmt::Write;
use std::time::Instant;

fn twist(xs: &mut [u8], len: usize, offset: usize) {
    if len == 0 { return; }

    let size = xs.len();
    let end = offset + len;
    if end >= size {
        // Range is split across end of slice.
        // We reverse the full range by dividing it into three parts.
        // The middle part is reversed in place. The two end parts
        // (which are chosen to have the same length) are reversed and
        // then swapped with each other. This reverse-and-swap is faster
        // than elementwise swapping because slice reverse is fairly
        // well optimized in the std lib.

        // Find short section, either [offset..] or [..end]
        let end = end % size;
        let (front,back) = xs.split_at_mut(offset);
        let (a,b,c) = if end < size - offset {
            // Wrapped section is the short one
            // The three regions are
            //   offset..offset+end     -- A. reverse and swap with C
            //   offset+end..size       -- B. reverse
            //   0..end                 -- C. reverse and swap with A
            let (p,q) = back.split_at_mut(end);
            (p, q, &mut front[..end])
        } else {
            // Start section (from offset to end) is the short one
            // The three regions are
            //   offset..size                   -- A. reverse and swap with C
            //   0..(end - (size - offset))     -- B. reverse
            //   (end - (size - offset))..end   -- C. reverse and swap with A
            let mid = end - (size - offset);
            let (p,q) = front[..end].split_at_mut(mid);
            (back, p, q)
        };
        a.reverse();
        b.reverse();
        c.reverse();

        // nightly has swap_with_slice, but I'm not using nightly!
        let tmp = c.to_vec();
        c.copy_from_slice(a);
        a.copy_from_slice(&tmp);

    } else {
        // Range is not split
        xs[offset..end].reverse();
    }
}

fn apply_lengths(size: usize, lengths: &[usize], rounds: u32) -> Vec<u8> {
    let mut elems: Vec<u8> = (0..size).map(|x| x as u8).collect();
    let mut skip = 0;
    let mut offset = 0;

    for _ in 0..rounds {
        for &l in lengths {
            twist(&mut elems, l, offset);
            offset = (offset + l + skip) % size;
            skip += 1;
        }
    }
    elems
}

const SUGAR: [usize; 5] = [17, 31, 73, 47, 23];

fn prep_key(keystr: &str) -> Vec<usize> {
    let mut key = Vec::with_capacity(keystr.len() + SUGAR.len());
    keystr.bytes().for_each(|b| key.push(b as usize));
    key.extend(&SUGAR);
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
    let part1 = xs[0] as u32 * xs[1] as u32;

    let part2 = knot_hash(input.trim());

    (part1, part2)
}

pub fn run(input: &str) {
    let (part1, part2) = solve(input);
    println!("the single round test value is {}", part1);
    println!("the knot hash is {}", part2);
    
    const REPS: u32 = 1000;
    let now = Instant::now();
    for _ in 0..REPS {
        knot_hash("AoC 2017");
    }
    let e = now.elapsed();
    println!("{} hashes in {}.{:03}s", REPS, e.as_secs(), e.subsec_nanos() / 1000000);
    let hps = REPS as f64 / (e.as_secs() as f64 + e.subsec_nanos() as f64 * 1e-9);
    println!("{:.0} hashes/sec", hps);
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
