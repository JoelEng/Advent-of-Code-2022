use hashbrown::HashSet;
use onig::Regex;

type Pos = (usize, usize);

#[aoc::main(14)]
fn main(input: &str) -> (usize, usize) {
    let (mut map, max) = rocks(input).unwrap();
    let rock_count = map.len();
    let mut p1 = 0;

    loop {
        let s = sand(&map, max);
        if s.1 == max && p1 == 0 {
            p1 = map.len() - rock_count;
        }
        map.insert(s);
        if s == (500, 0) {
            break;
        }
    }

    (p1, map.len() - rock_count)
}

fn sand(map: &HashSet<Pos>, depth: usize) -> Pos {
    let (mut x, mut y) = (500, 0);
    while y < depth {
        if !map.contains(&(x, y + 1)) {
            y += 1;
        } else if !map.contains(&(x - 1, y + 1)) {
            x -= 1;
            y += 1;
        } else if !map.contains(&(x + 1, y + 1)) {
            x += 1;
            y += 1;
        }

        if map.contains(&(x, y + 1))
            && map.contains(&(x - 1, y + 1))
            && map.contains(&(x + 1, y + 1))
        {
            return (x, y);
        }
    }
    (x, y)
}

fn rocks(input: &str) -> Option<(HashSet<Pos>, usize)> {
    let re = Regex::new(r"(\d+),(\d+) -> (?=(\d+),(\d+))").unwrap();
    let mut map: HashSet<Pos> = HashSet::new();
    let mut max = 0;
    for r in re.captures_iter(input) {
        let (x0, y0): Pos = (r.at(1)?.parse().ok()?, r.at(2)?.parse().ok()?);
        let (x1, y1): Pos = (r.at(3)?.parse().ok()?, r.at(4)?.parse().ok()?);
        let (x_min, x_max) = (x0.min(x1), x0.max(x1));
        let (y_min, y_max) = (y0.min(y1), y0.max(y1));
        if y_max > max {
            max = y_max
        }
        for x in x_min..=x_max {
            for y in y_min..=y_max {
                map.insert((x, y));
            }
        }
    }
    Some((map, max + 1))
}
