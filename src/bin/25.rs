#[aoc::main(25)]
fn main(input: &str) -> (String, i32) {
    let snafus: Vec<&str> = input.lines().collect();
    let sum = snafus.iter().map(|n| snafu_to_int(n)).sum();
    (int_to_snafu(sum), 0)
}

fn snafu_to_int(snafu: &str) -> usize {
    snafu.chars().fold(0, |acc, s| {
        acc * 5 + "=-012".chars().position(|n| n == s).unwrap() - 2
    })
}

fn int_to_snafu(int: usize) -> String {
    if int == 0 {
        String::new()
    } else {
        int_to_snafu((int + 2) / 5) + &["0", "1", "2", "=", "-"][int % 5].to_string()
    }
}
