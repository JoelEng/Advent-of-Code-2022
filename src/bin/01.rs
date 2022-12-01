#[aoc::main(01)]
fn main(input: &str) -> (u32, u32) {
    let mut elves: Vec<u32> = input
        .split("\n\n")
        .map(|e| e.lines().map(|l| l.parse::<u32>().unwrap()).sum())
        .collect();

    elves.sort_by(|a, b| b.cmp(a));
    (elves[0], elves[0..3].iter().sum())
}
