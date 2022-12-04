#[aoc::main(04)]
fn main(input: &str) -> (usize, usize) {
    let pairs = get_pairs(input);

    let (mut p1, mut p2) = (0, 0);
    for (a, b) in pairs {
        if (a.0 <= b.0 && a.1 >= b.1) || (b.0 <= a.0 && b.1 >= a.1) {
            p1 += 1;
        }
        if b.0 <= a.1 && a.0 <= b.1 {
            p2 += 1;
        }
    }
    (p1, p2)
}

// This parsing is not as pretty as scan_fmt, but much much faster
fn get_pairs(input: &str) -> Vec<((u32, u32), (u32, u32))> {
    input
        .lines()
        .filter_map(|l| {
            let (a, b) = l.split_once(",")?;
            let ((a0, a1), (b0, b1)) = (a.split_once("-")?, b.split_once("-")?);
            Some((
                (a0.parse().ok()?, a1.parse().ok()?),
                (b0.parse().ok()?, b1.parse().ok()?),
            ))
        })
        .collect()
}
