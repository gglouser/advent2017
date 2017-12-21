use std::collections::HashMap;

#[derive(Clone, PartialEq, Eq, Hash, Debug)]
struct Pattern(Vec<Vec<bool>>);

impl Pattern {
    fn size(&self) -> usize {
        self.0.len()
    }

    fn count_on(&self) -> usize {
        self.0.iter()
            .map(|r| r.iter().filter(|&&x| x).count())
            .sum()
    }

    fn flip(&self) -> Pattern {
        Pattern(self.0.iter().rev().cloned().collect())
    }

    fn transpose(&self) -> Pattern {
        let num_cols = self.0[0].len();
        let mut new = Vec::new();
        new.resize(num_cols, Vec::new());
        for row in self.0.iter() {
            new.iter_mut().zip(row.iter())
                .for_each(|(v,&x)| v.push(x));
        }
        Pattern(new)
    }

    fn view(&self, size: usize, row: usize, col: usize) -> Pattern {
        Pattern(self.0[row..row+size].iter()
            .map(|r| r[col..col+size].to_vec())
            .collect())
    }
}

fn parse_pattern(s: &str) -> Pattern {
    Pattern(s.split('/')
        .map(|row| row.bytes().map(|b| b == b'#').collect())
        .collect())
}

type PatMap = HashMap<Pattern, Pattern>;

fn parse_input(s: &str) -> PatMap {
    s.lines().map(|line| {
            let mut parts = line.split(" => ").map(|p| parse_pattern(p));
            (parts.next().unwrap(), parts.next().unwrap())
        }).collect()
}

fn lookup<'a>(patmap: &'a PatMap, p: &Pattern) -> &'a Pattern {
    if let Some(q) = patmap.get(p) { return q; }
    let p = p.flip();
    if let Some(q) = patmap.get(&p) { return q; }
    let p = p.transpose();
    if let Some(q) = patmap.get(&p) { return q; }
    let p = p.flip();
    if let Some(q) = patmap.get(&p) { return q; }
    let p = p.transpose();
    if let Some(q) = patmap.get(&p) { return q; }
    let p = p.flip();
    if let Some(q) = patmap.get(&p) { return q; }
    let p = p.transpose();
    if let Some(q) = patmap.get(&p) { return q; }
    let p = p.flip();
    if let Some(q) = patmap.get(&p) { return q; }
    panic!("pattern not found! {:?}", p);
}

fn merge_h(pats: Vec<&Pattern>) -> Pattern {
    let mut new_pat: Vec<Vec<bool>> = Vec::new();
    new_pat.resize(pats[0].size(), Vec::new());
    for p in pats {
        new_pat.iter_mut().zip(p.0.iter())
            .for_each(|(v,r)| v.extend_from_slice(r));
    }
    Pattern(new_pat)
}

fn merge_v(pats: Vec<Pattern>) -> Pattern {
    Pattern(pats.into_iter().flat_map(|p| p.0).collect())
}

fn enhance(patmap: &PatMap, pat: Pattern) -> Pattern {
    let size = pat.size();
    let chunk_size = if size % 2 == 0 { 2 } else { 3 };
    let pat_col = (0..size/chunk_size).map(|ch_r| {
        let pat_row = (0..size/chunk_size).map(|ch_c| {
            let v = pat.view(chunk_size, chunk_size*ch_r, chunk_size*ch_c);
            lookup(patmap, &v)
        }).collect::<Vec<_>>();
        merge_h(pat_row)
    }).collect::<Vec<_>>();
    merge_v(pat_col)
}

fn generate(patmap: &PatMap, iters: u32) -> Pattern{
    let mut pat = parse_pattern(".#./..#/###");
    for _ in 0..iters {
        pat = enhance(patmap, pat);
    }
    pat
}

fn solve(input: &str) -> (usize, usize) {
    let patmap = parse_input(input);
    let part1 = generate(&patmap, 5).count_on();
    let part2 = generate(&patmap, 18).count_on();
    (part1, part2)
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
        ../.# => ##./#../...\n\
        .#./..#/### => #..#/..../..../#..#\n";

    #[test]
    fn parsing() {
        let mut x = HashMap::new();
        x.insert(Pattern(vec![vec![false,false],
                              vec![false,true]]),
                 Pattern(vec![vec![true, true, false],
                              vec![true, false,false],
                              vec![false,false,false]]));
        x.insert(Pattern(vec![vec![false,true, false],
                              vec![false,false,true],
                              vec![true, true, true]]),
                 Pattern(vec![vec![true, false,false, true],
                              vec![false,false,false,false],
                              vec![false,false,false,false],
                              vec![true, false,false,true]]));
        assert_eq!(x, parse_input(EXAMPLE));
    }

    #[test]
    fn example1() {
        let patmap = parse_input(EXAMPLE);
        let part1 = generate(&patmap, 2).count_on();
        assert_eq!(12, part1);
    }

    #[test]
    fn real_input() {
        let input = include_str!("../inputs/day21.txt");
        let x = solve(&input);
        assert_eq!(include_str!("../outputs/day21.txt"),
                   format!("{:?}", x));
    }
}
