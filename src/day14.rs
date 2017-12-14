use day10::knot_hash;

fn solve(input: &str) -> (u32, u32) {

    let mut bits = 0;
    let mut grid: Vec<Vec<bool>> = Vec::new();
    for i in 0..128 {
        let mut row = String::new();
        let h = knot_hash(&format!("{}-{}", input, i));
        for n in h.iter() {
            bits += n.count_ones();
            row += &format!("{:08b}", n);
        }
        assert!(row.len() == 128);
        grid.push(row.bytes().map(|b| b == b'1').collect());
    }

    let mut regions = 0;
    let mut nodes = Vec::new();
    for row in 0..128 {
        for col in 0..128 {
            if grid[row][col] {
                nodes.push((row, col));
                while let Some((r,c)) = nodes.pop() {
                    if !grid[r][c] { continue; }
                    grid[r][c] = false;
                    if r < 127 { nodes.push((r+1,c)); }
                    if r > 0   { nodes.push((r-1,c)); }
                    if c < 127 { nodes.push((r,c+1)); }
                    if c > 0   { nodes.push((r,c-1)); }
                }
                regions += 1;
            }
        }
    }

    (bits, regions)
}

pub fn run(input: &str) {
    let (part1, part2) = solve(input.trim());
    println!("the number of used squares is {}", part1);
    println!("the total number of regions is {}", part2);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example1() {
        let (part1,part2) = solve("flqrgnkx");
        assert_eq!(8108, part1);
        assert_eq!(1242, part2);
    }

    #[test]
    fn real_input() {
        let input = include_str!("../inputs/day14.txt");
        let x = solve(&input.trim());
        assert_eq!(include_str!("../outputs/day14.txt"),
                   format!("{:?}", x));
    }
}
