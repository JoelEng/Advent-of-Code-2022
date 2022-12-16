use itertools::Itertools;

#[aoc::main(15)]
fn main(input: &str) -> (i64, i64) {
    let row = aox(2000000, 10);
    let sensors = input
        .replace(&['=', ',', ':'][..], " ")
        .split_whitespace()
        .filter_map(|w| w.parse().ok())
        .tuples()
        .map(|(sx, sy, bx, by)| {
            (
                (sx, sy),
                (bx, by),
                ((sx - bx) as i64).abs() + ((sy - by) as i64).abs(),
            )
        })
        .collect();
    (p1(&sensors, row), p2(&sensors, row * 2))
}

fn p1(sensors: &Vec<((i64, i64), (i64, i64), i64)>, row: i64) -> i64 {
    let s_row: Vec<_> = sensors
        .iter()
        .filter(|(s, _, dist)| (s.1 - row).abs() <= *dist)
        .map(|(s, b, dist)| {
            let w = dist - (s.1 - row).abs();
            (s.0 - w, s.0 + w, if b.1 == row { Some(b.0) } else { None })
        })
        .sorted()
        .collect();
    let mut ans = 0;
    let mut at = s_row.iter().map(|s| s.0).min().unwrap();
    for (s, e, b) in &s_row {
        let from = at.max(*s);
        if let Some(b) = b {
            ans -= if *b >= at { 1 } else { 0 };
        }
        if e > &from {
            ans += (e - from).abs() + 1;
            at = e + 1;
        }
    }
    ans
}

fn p2(sensors: &Vec<((i64, i64), (i64, i64), i64)>, max: i64) -> i64 {
    for (s, _, dist) in sensors {
        let others: Vec<&((i64, i64), (i64, i64), i64)> = sensors
            .iter()
            .filter(|(o, _, d)| manhattan(s, o) <= dist + d + 1 && s != o)
            .collect();
        for x in 0..dist + 1 {
            let y = dist + 1 - x;
            'pos: for (x, y) in [(x, y), (x, -y), (-x, y), (-x, -y)] {
                let p = (s.0 + x, s.1 + y);
                if p.0 < 0 || p.1 < 0 || p.0 > max || p.1 > max {
                    continue;
                }
                for (s, _, dist_b) in &others {
                    if manhattan(s, &p) <= *dist_b {
                        continue 'pos;
                    }
                }
                return p.0 * 4000000 + p.1;
            }
        }
    }
    unreachable!()
}

fn manhattan(a: &(i64, i64), b: &(i64, i64)) -> i64 {
    (a.0 - b.0).abs() + (a.1 - b.1).abs()
}
