use itertools::Itertools;

#[aoc::main(06)]
fn main(input: &str) -> (usize, usize) {
    (solve(input, 4), solve(input, 14))
}

fn solve(input: &str, z: usize) -> usize {
    for (i, w) in input.as_bytes().windows(z).enumerate() {
        if w.iter().tuple_combinations().all(|(a, b)| a != b) {
            return i + z;
        }
    }
    return 0;
}
