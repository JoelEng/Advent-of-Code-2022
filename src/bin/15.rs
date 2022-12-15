use itertools::Itertools;

const ROW: i64 = 2000000; // Example: 10, Real: 2000000
const MAX: i64 = ROW * 2;

#[aoc::main(15)]
fn main(input: &str) -> (usize, i64) {
    let input = input.replace(&['=', ',', ':'][..], " ");
    let sensors: Vec<((i64, i64), (i64, i64), i64)> = input
        .lines()
        .map(|l| {
            l.split_whitespace()
                .filter_map(|w| w.parse().ok())
                .collect_tuple()
                .unwrap()
        })
        .map(|(sx, sy, bx, by)| {
            (
                (sx, sy),
                (bx, by),
                ((sx - bx) as i64).abs() + ((sy - by) as i64).abs(),
            )
        })
        .collect();

    let min_x = sensors.iter().map(|(s, _, dist)| s.0 - dist).min().unwrap();
    let max_x = sensors.iter().map(|(s, _, dist)| s.0 + dist).max().unwrap();

    let mut p1 = 0;
    for x in min_x..max_x {
        let p = (x, ROW);
        for (s, b, dist_b) in &sensors {
            let dist_p = (s.0 - p.0).abs() + (s.1 - p.1).abs();
            if dist_p <= *dist_b && p != *b {
                p1 += 1;
                break;
            }
        }
    }

    (p1, p2(&sensors))
}

fn p2(sensors: &Vec<((i64, i64), (i64, i64), i64)>) -> i64 {
    for (s, _, dist) in sensors {
        let check = dist + 1;
        for x in 0..=check {
            let y = check - x;
            'pos: for (x, y) in [(x, y), (x, -y), (-x, y), (-x, -y)] {
                let p = (s.0 + x, s.1 + y);
                if p.0 < 0 || p.1 < 0 || p.0 > MAX || p.1 > MAX {
                    continue;
                }
                for (s, _, dist_b) in sensors {
                    let dist_p = (s.0 - p.0).abs() + (s.1 - p.1).abs();
                    if dist_p <= *dist_b {
                        continue 'pos;
                    }
                }
                return p.0 * 4000000 + p.1;
            }
        }
    }
    unreachable!()
}
