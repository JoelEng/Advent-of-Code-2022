use hashbrown::{HashMap, HashSet};

#[aoc::main(24)]
fn main(input: &str) -> (i32, i32) {
    let mut valley: HashMap<(i32, i32), Vec<(i32, i32)>> = input
        .lines()
        .enumerate()
        .map(|(y, l)| {
            l.chars().enumerate().filter_map(move |(x, c)| {
                let pos = (x as i32, y as i32);
                match c {
                    '.' => None,
                    '#' => Some((pos, vec![(0, 0)])),
                    '>' => Some((pos, vec![(1, 0)])),
                    '<' => Some((pos, vec![(-1, 0)])),
                    '^' => Some((pos, vec![(0, -1)])),
                    'v' => Some((pos, vec![(0, 1)])),
                    _ => unreachable!(),
                }
            })
        })
        .flatten()
        .collect();
    let xmax = valley.keys().map(|b| b.0).max().unwrap();
    let ymax = valley.keys().map(|b| b.1).max().unwrap();

    let (p1, start) = find_path((1, 0), ymax, &mut valley, xmax, ymax);

    let p2 = p1 + find_path(start, 0, &mut valley, xmax, ymax).0;
    let p2 = p2 + find_path((1, 0), ymax, &mut valley, xmax, ymax).0;

    (p1, p2)
}

fn find_path(
    start: (i32, i32),
    y_end: i32,
    valley: &mut HashMap<(i32, i32), Vec<(i32, i32)>>,
    xmax: i32,
    ymax: i32,
) -> (i32, (i32, i32)) {
    let mut current = HashSet::new();
    current.insert(start);
    let mut ans = 1;
    loop {
        storm(valley, xmax, ymax);
        current = walk(&current, &valley, xmax, ymax);
        let x = current.iter().map(|p| p.1);
        if x.clone().max().unwrap() == y_end || x.min().unwrap() == y_end {
            break;
        }
        ans += 1;
    }
    (ans, *current.iter().max_by_key(|p| p.1).unwrap())
}

fn walk(
    current: &HashSet<(i32, i32)>,
    valley: &HashMap<(i32, i32), Vec<(i32, i32)>>,
    xmax: i32,
    ymax: i32,
) -> HashSet<(i32, i32)> {
    let mut new = HashSet::new();
    for pos in current {
        for dir in [(0, 0), (1, 0), (-1, 0), (0, 1), (0, -1)] {
            let pos = (pos.0 + dir.0, pos.1 + dir.1);
            if pos.0 < 0 || pos.1 < 0 || pos.0 > xmax || pos.1 > ymax {
                continue;
            }
            if !valley.contains_key(&pos) {
                new.insert(pos);
            }
        }
    }
    new
}

fn storm(valley: &mut HashMap<(i32, i32), Vec<(i32, i32)>>, xmax: i32, ymax: i32) {
    let mut new = HashMap::new();
    for (pos, vec) in &*valley {
        if vec.contains(&(0, 0)) {
            new.insert(*pos, vec.to_owned());
            continue;
        }
        for dir in vec {
            let mut blizzard = (pos.0 + dir.0, pos.1 + dir.1);
            if let Some(prev) = valley.get(&blizzard) {
                if prev.contains(&(0, 0)) {
                    if blizzard.0 == 0 {
                        blizzard.0 = xmax - 1;
                    } else if blizzard.1 == 0 {
                        blizzard.1 = ymax - 1;
                    } else if blizzard.0 == xmax {
                        blizzard.0 = 1;
                    } else if blizzard.1 == ymax {
                        blizzard.1 = 1;
                    }
                }
            }
            new.entry(blizzard).or_default().push(*dir);
        }
    }
    *valley = new;
}
