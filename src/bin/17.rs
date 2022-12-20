use hashbrown::{HashMap, HashSet};

const CHECK_HEIGHT: usize = 100;
const ROCKS: usize = 1000000000000;

#[aoc::main(17)]
fn main(input: &str) -> (usize, usize) {
    let mut jets = input.chars().map(|c| if c == '>' { 1 } else { -1 }).cycle();
    let rocks: [Vec<(usize, usize)>; 5] = [
        vec![(0, 0), (1, 0), (2, 0), (3, 0)],
        vec![(1, 0), (0, 1), (1, 1), (2, 1), (1, 2)],
        vec![(0, 0), (1, 0), (2, 0), (2, 1), (2, 2)],
        vec![(0, 0), (0, 1), (0, 2), (0, 3)],
        vec![(0, 0), (1, 0), (0, 1), (1, 1)],
    ];
    let rocks_len = rocks.len();
    let mut rocks = rocks.iter().cycle().enumerate();
    let mut chamber: HashSet<(usize, usize)> = HashSet::new();

    let mut tops = HashMap::new();

    let mut p1 = 0;
    let mut height = 0;
    let mut diff = 0;
    let mut diff_height = 0;
    while let Some((i, r)) = rocks.next() {
        if diff == 0 && i % rocks_len == 0 && height > CHECK_HEIGHT {
            let mut top = 0;
            for y in 0..CHECK_HEIGHT {
                for x in 0..7 {
                    if chamber.contains(&(x, height - y)) {
                        top += y * 10 + x;
                    }
                }
            }
            if let Some((prev_i, prev_height)) = tops.insert(top, (i, height)) {
                diff_height = height - prev_height;
                diff = i - prev_i;
            }
        }
        drop(r, &mut chamber, &mut jets);
        height = chamber.iter().map(|(_, y)| y).max().unwrap() + 1;
        if i == 2022 - 1 {
            p1 = height;
        }
        if p1 != 0 && diff != 0 {
            break;
        }
    }
    let diff_height = diff_height * ((ROCKS - 2022) / diff);

    for _ in 0..(ROCKS - 2022) % diff {
        let r = rocks.next().unwrap().1;
        drop(r, &mut chamber, &mut jets);
        height = chamber.iter().map(|(_, y)| y).max().unwrap() + 1;
    }
    let p2 = height + diff_height;

    (p1, p2)
}

fn drop<T: Iterator<Item = isize>>(
    rock: &Vec<(usize, usize)>,
    chamber: &mut HashSet<(usize, usize)>,
    jets: &mut T,
) {
    let mut rock: Vec<(usize, usize)> = rock
        .clone()
        .iter()
        .map(|(x, y)| {
            (
                x + 2,
                y + 4 + chamber.iter().map(|(_, y)| *y as isize).max().unwrap_or(-1) as usize,
            )
        })
        .collect();
    loop {
        if let Some(r) = sideways(jets.next().unwrap(), &rock, chamber) {
            rock = r;
        }
        if let Some(r) = downwards(&rock, chamber) {
            rock = r;
        } else {
            break;
        }
    }
    chamber.extend(rock);
}

fn downwards(
    rock: &Vec<(usize, usize)>,
    chamber: &HashSet<(usize, usize)>,
) -> Option<Vec<(usize, usize)>> {
    let mut new_rock = rock.clone();
    for (_, y) in &mut new_rock {
        *y = y.checked_sub(1)?;
    }
    new_rock
        .iter()
        .all(|r| !chamber.contains(r))
        .then_some(new_rock)
}

fn sideways(
    jet: isize,
    rock: &Vec<(usize, usize)>,
    chamber: &HashSet<(usize, usize)>,
) -> Option<Vec<(usize, usize)>> {
    let mut new_rock = rock.clone();
    let max_x = rock.iter().max_by_key(|(x, _)| x)?.0;
    if max_x < 6 || jet < 0 {
        for (x, _) in &mut new_rock {
            *x = x.checked_add_signed(jet)?;
        }
    }
    new_rock
        .iter()
        .all(|r| !chamber.contains(r))
        .then_some(new_rock)
}
