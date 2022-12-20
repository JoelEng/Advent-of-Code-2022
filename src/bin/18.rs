use hashbrown::HashSet;
use itertools::Itertools;
use std::collections::VecDeque;

const DIRS: [(i32, i32, i32); 6] = [
    (1, 0, 0),
    (0, 1, 0),
    (0, 0, 1),
    (-1, 0, 0),
    (0, -1, 0),
    (0, 0, -1),
];

#[aoc::main(18)]
fn main(input: &str) -> (i32, i32) {
    let mut droplets: HashSet<(i32, i32, i32)> = input
        .replace(",", " ")
        .split_whitespace()
        .filter_map(|w| w.parse().ok())
        .tuples()
        .collect();
    let p1 = sides(&mut droplets);

    let xmax = droplets.iter().map(|d| d.0).max().unwrap();
    let ymax = droplets.iter().map(|d| d.1).max().unwrap();
    let zmax = droplets.iter().map(|d| d.2).max().unwrap();
    let water = flood_fill(&droplets, xmax, ymax, zmax);

    for x in 0..=xmax {
        for y in 0..=ymax {
            for z in 0..=zmax {
                if !water.contains(&(x, y, z)) {
                    droplets.insert((x, y, z));
                }
            }
        }
    }

    (p1, sides(&droplets))
}

fn sides(input: &HashSet<(i32, i32, i32)>) -> i32 {
    let mut droplets = HashSet::new();
    let mut ans = 0;
    for drop in input {
        droplets.insert(*drop);
        ans += 6;
        for (x, y, z) in DIRS {
            let d = (drop.0 + x, drop.1 + y, drop.2 + z);
            if droplets.get(&d).is_some() {
                ans -= 2;
            }
        }
    }
    ans
}

fn flood_fill(
    droplets: &HashSet<(i32, i32, i32)>,
    xmax: i32,
    ymax: i32,
    zmax: i32,
) -> HashSet<(i32, i32, i32)> {
    let mut water = HashSet::new();
    let mut q = VecDeque::new();
    q.push_back((0, 0, 0));
    while let Some((x, y, z)) = q.pop_front() {
        if !droplets.contains(&(x, y, z))
            && !water.contains(&(x, y, z))
            && x >= 0
            && x <= xmax
            && y >= 0
            && y <= ymax
            && z >= 0
            && z <= zmax
        {
            water.insert((x, y, z));
            for (x1, y1, z1) in DIRS {
                q.push_back((x + x1, y + y1, z + z1));
            }
        }
    }
    water
}
