use scan_fmt::scan_fmt;

#[aoc::main(04)]
fn main(input: &str) -> (usize, usize) {
    let pairs: Vec<(u32, u32, u32, u32)> = input
        .lines()
        .map(|l| scan_fmt!(l, "{}-{},{}-{}", u32, u32, u32, u32).unwrap())
        .collect();

    let (mut p1, mut p2) = (0, 0);
    for (e00, e01, e10, e11) in pairs {
        if (e00 <= e10 && e01 >= e11) || (e10 <= e00 && e11 >= e01) {
            p1 += 1;
        }
        for a in e00..=e01 {
            if (e10..=e11).contains(&a) {
                p2 += 1;
                break;
            }
        }
    }
    (p1, p2)
}
