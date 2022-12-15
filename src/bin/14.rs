use hashbrown::HashSet;
use onig::Regex;

type Pos = (usize, usize);

#[aoc::main(14)]
fn main(input: &str) -> (usize, usize) {
    let (mut map, max) = rocks(input).unwrap();
    let mut fall = vec![];
    let rock_count = map.len();
    let mut p1 = 0;
    loop {
        let s = sand(&map, max, &mut fall);
        if p1 == 0 && s.1 == max {
            p1 = map.len() - rock_count;
        }
        map.insert(s);
        if s == (500, 0) {
            break;
        }
    }
    (p1, map.len() - rock_count)
}

fn sand(map: &HashSet<Pos>, depth: usize, fall: &mut Vec<(usize, usize)>) -> Pos {
    let (mut x, mut y) = fall.pop().unwrap_or((500, 0));
    while y < depth {
        y += 1;
        if let Some(a) = [x, x - 1, x + 1].iter().find(|x| !map.contains(&(**x, y))) {
            x = *a;
            fall.push((x, y));
        } else {
            return (x, y - 1);
        }
    }
    (x, y)
}

fn rocks(input: &str) -> Option<(HashSet<Pos>, usize)> {
    let re = Regex::new(r"(\d+),(\d+) -> (?=(\d+),(\d+))").ok()?;
    let mut map = HashSet::new();
    let mut max = 0;
    for r in re.captures_iter(input) {
        let (x0, y0): Pos = (r.at(1)?.parse().ok()?, r.at(2)?.parse().ok()?);
        let (x1, y1): Pos = (r.at(3)?.parse().ok()?, r.at(4)?.parse().ok()?);
        let (y_min, y_max) = (y0.min(y1), y0.max(y1));
        max = max.max(y_max);
        for x in x0.min(x1)..=x0.max(x1) {
            for y in y_min..=y_max {
                map.insert((x, y));
            }
        }
    }
    Some((map, max + 1))
}
