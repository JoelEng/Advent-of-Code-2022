use itertools::Itertools;
use memoize::memoize;

type Blueprint = (u32, u32, (u32, u32), (u32, u32));
type Resources = [u32; 4];

#[aoc::main(19)]
fn main(input: &str) -> (u32, u32) {
    let blueprints: Vec<Blueprint> = input
        .split_whitespace()
        .filter_map(|w| w.parse().ok())
        .chunks(6)
        .into_iter()
        .filter_map(|mut l| {
            Some((
                l.next()?,
                l.next()?,
                (l.next()?, l.next()?),
                (l.next()?, l.next()?),
            ))
        })
        .collect();

    let mut p1 = 0;
    for (i, b) in blueprints.iter().enumerate() {
        let ans = geodes([1, 0, 0, 0], [0, 0, 0, 0], 24, *b);
        // println!("blueprint {}: {}", i + 1, ans);
        p1 += ans * (i as u32 + 1);
    }
    let p2 = blueprints
        .iter()
        .take(3)
        .map(|b| geodes([1, 0, 0, 0], [0, 0, 0, 0], 32, *b))
        .product();

    (p1, p2)
}

#[memoize]
fn geodes(robots: Resources, resources: Resources, time: u32, b: Blueprint) -> u32 {
    // println!("geoding {:?}", robots);
    if time == 1 {
        return robots[3];
    }
    let new_resources = sum_resources(&resources, &robots);
    if resources[0] >= b.3 .0 && resources[2] >= b.3 .1 {
        let (mut rob, mut res) = (robots.clone(), new_resources.clone());
        rob[3] += 1;
        res[0] -= b.3 .0;
        res[2] -= b.3 .1;
        return robots[3] + geodes(rob, res, time - 1, b);
    }
    let mut possible = vec![geodes(robots, new_resources, time - 1, b)];
    let max_ore = b.0.max(b.1).max(b.2 .0).max(b.3 .0);
    let max_clay = b.2 .1;
    let max_obs = b.3 .1;
    let prev_ore = resources[0] - robots[0];
    let prev_clay = resources[1] - robots[1];
    if resources[0] >= b.0 && robots[0] <= max_ore && prev_ore < b.0 {
        let (mut rob, mut res) = (robots.clone(), new_resources.clone());
        rob[0] += 1;
        res[0] -= b.0;
        possible.push(geodes(rob, res, time - 1, b));
    }
    if resources[0] >= b.1 && robots[1] <= max_clay && prev_ore < b.1 {
        let (mut rob, mut res) = (robots.clone(), new_resources.clone());
        rob[1] += 1;
        res[0] -= b.1;
        possible.push(geodes(rob, res, time - 1, b));
    }
    if resources[0] >= b.2 .0 && resources[1] >= b.2 .1 && robots[2] <= max_obs
    // && (prev_ore < b.2 .0 || prev_clay < b.2 .1)
    {
        let (mut rob, mut res) = (robots.clone(), new_resources.clone());
        rob[2] += 1;
        res[0] -= b.2 .0;
        res[1] -= b.2 .1;
        possible.push(geodes(rob, res, time - 1, b));
    }

    robots[3] + *possible.iter().max().unwrap()
}

fn sum_resources(a: &[u32; 4], b: &[u32; 4]) -> Resources {
    let mut res = [0; 4];
    for (i, (a, b)) in a.iter().zip(b).enumerate() {
        res[i] = a + b;
    }
    res
}
