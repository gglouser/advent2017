fn spinlock(skip: u32, reps: u32) -> u32 {
    let mut state: Vec<u32> = Vec::with_capacity(reps as usize + 1);
    let mut pos = 0;
    state.push(0);
    for i in 1..reps+1 {
        pos = (pos + skip) % i + 1;
        state.insert(pos as usize, i);
    }
    state[(pos as usize + 1) % state.len()]
}

fn spinlock2(skip: u32, reps: u32) -> u32 {
    let mut pos = 0;
    let mut target = 0;
    for i in 2..reps+1 {
        pos = (pos + skip + 1) % i;
        if pos == 0 {
            target = i;
        }
    }
    target
}

fn solve(input: &str) -> (u32, u32) {
    let input = input.trim().parse().unwrap();
    let part1 = spinlock(input, 2017);
    let part2 = spinlock2(input, 50_000_000);
    (part1, part2)
}

pub fn run(input: &str) {
    let (part1, part2) = solve(input);
    println!("the value after 2017 is {}", part1);
    println!("the value after 0 is {}", part2);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example1() {
        assert_eq!(638, spinlock(3, 2017));
    }

    #[test]
    fn real_input() {
        let input = include_str!("../inputs/day17.txt");
        let x = solve(&input);
        assert_eq!(include_str!("../outputs/day17.txt"),
                   format!("{:?}", x));
    }
}
