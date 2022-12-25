use hashbrown::{HashMap, HashSet};

#[aoc::main(23)]
fn main(input: &str) -> (i32, i32) {
    let mut elves: HashSet<(i32, i32)> = input
        .lines()
        .enumerate()
        .map(|(r, l)| {
            l.chars()
                .enumerate()
                .filter(|(_, c)| *c == '#')
                .map(move |(c, _)| (c as i32, r as i32))
        })
        .flatten()
        .collect();

    let north = vec![(0, -1), (1, -1), (-1, -1)];
    let south = vec![(0, 1), (1, 1), (-1, 1)];
    let west = vec![(-1, 0), (-1, 1), (-1, -1)];
    let east = vec![(1, 0), (1, 1), (1, -1)];
    let mut dirs = vec![north, south, west, east];

    let mut proposed = HashMap::new();
    let mut failed = HashSet::new();

    let mut prev_elves = elves.clone();

    let mut p1 = 0;
    let mut p2 = 1;
    loop {
        for (x, y) in &elves {
            if dirs
                .iter()
                .flatten()
                .all(|a| !elves.contains(&(x + a.0, y + a.1)))
            {
                continue;
            }
            for d in &dirs {
                let p = (x + d[0].0, y + d[0].1);
                if d.iter().all(|d| !elves.contains(&(x + d.0, y + d.1))) {
                    if !failed.contains(&p) {
                        if let Some(_) = proposed.insert(p, (*x, *y)) {
                            proposed.remove(&p);
                            failed.insert(p);
                        }
                    }
                    break;
                }
            }
        }
        for (p, e) in proposed.drain() {
            elves.remove(&e);
            elves.insert(p);
        }
        failed.clear();
        let d = dirs.remove(0);
        dirs.push(d);

        if p2 == 10 {
            p1 = count_ground(&elves);
        }

        if prev_elves == elves {
            break;
        }
        prev_elves = elves.clone();

        p2 += 1;
    }

    (p1, p2)
}

fn count_ground(elves: &HashSet<(i32, i32)>) -> i32 {
    let xmin = elves.iter().map(|e| e.0).min().unwrap();
    let xmax = elves.iter().map(|e| e.0).max().unwrap();
    let ymin = elves.iter().map(|e| e.1).min().unwrap();
    let ymax = elves.iter().map(|e| e.1).max().unwrap();
    let mut ans = 0;
    for x in xmin..=xmax {
        for y in ymin..=ymax {
            if !elves.contains(&(x, y)) {
                ans += 1;
            }
        }
    }
    ans
}
