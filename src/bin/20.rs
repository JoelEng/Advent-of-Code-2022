#[aoc::main(20)]
fn main(input: &str) -> (i64, i64) {
    (solve(input, 1, 1), solve(input, 811589153, 10))
}

fn solve(input: &str, key: i64, count: usize) -> i64 {
    let file: Vec<i64> = input
        .lines()
        .map(|l| l.parse::<i64>().unwrap() * key)
        .collect();
    let mut idx: Vec<usize> = (0..file.len()).collect();

    for _ in 0..count {
        for (i, num) in file.iter().enumerate() {
            let pos = idx.iter().position(|n| *n == i).unwrap();
            idx.remove(pos);
            let new_i = (pos as i64 + num).rem_euclid(idx.len() as i64) as usize;
            idx.insert(new_i, i);
        }
    }

    let i0 = file.iter().position(|n| *n == 0).unwrap();
    let i = idx.iter().position(|n| *n == i0).unwrap();
    [1000, 2000, 3000]
        .iter()
        .map(|n| file[idx[(i + n) % idx.len()]])
        .sum()
}
