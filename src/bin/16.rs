use bit_set::BitSet;
use hashbrown::HashMap;
use memoize::memoize;
use std::cell::RefCell;

std::thread_local! {
  static VALVES : RefCell<HashMap<usize, (u32, HashMap<usize, u32>)>> = RefCell::new(HashMap::new());
}

#[aoc::main(16)]
fn main(input: &str) -> (u32, i32) {
    let input = input.replace(&['=', ';', ','][..], " ");
    let mut valves: HashMap<usize, (u32, HashMap<usize, u32>)> = input
        .lines()
        .map(|l| {
            let mut v = l
                .split_whitespace()
                .filter(|w| w.chars().all(|c| c.is_uppercase()))
                .map(|s| (to_usize(s), 1));
            let n = l.split_whitespace().find_map(|w| w.parse().ok()).unwrap();
            (v.next().unwrap().0, (n, v.collect()))
        })
        .collect();
    VALVES.with(|v| v.replace(valves.clone()));
    for (name, (_, children)) in &mut valves {
        *children = remove_broken(*name, BitSet::new());
        children.remove(name);
    }
    VALVES.with(|v| v.replace(valves));

    // p1 works on the actual problem, but not on the example input
    let p1 = pressure(to_usize("AA"), 1, BitSet::new());
    (p1, 0)
}

fn remove_broken(name: usize, visited: BitSet) -> HashMap<usize, u32> {
    let (_, children) = VALVES
        .with(|valves| valves.borrow().get(&name).cloned())
        .unwrap();
    let mut map = HashMap::new();

    let mut new_visited = visited.clone();
    new_visited.insert(name);
    for (name, distance) in children {
        let (pressure, _) = VALVES
            .with(|valves| valves.borrow().get(&name).cloned())
            .unwrap();
        if pressure != 0 {
            map.insert(name, distance);
            continue;
        }
        if !visited.contains(name) {
            map.extend(
                remove_broken(name, new_visited.clone())
                    .iter()
                    .map(|(k, v)| (*k, v + 1)),
            );
        }
    }
    map
}

#[memoize]
fn pressure(name: usize, time: u32, open: BitSet) -> u32 {
    if time >= 30 {
        return 0;
    }
    let (press, children) = VALVES
        .with(|valves| valves.borrow().get(&name).cloned())
        .unwrap();
    if time == 29 {
        return press;
    }
    let children: HashMap<&usize, &u32> =
        children.iter().filter(|v| !open.contains(*v.0)).collect();
    let walked = children
        .iter()
        .map(|(name, dist)| pressure(**name, time + *dist, open.clone()))
        .max()
        .unwrap_or(0);

    if press == 0 {
        // Happens once, when entering from AA
        return walked;
    }

    let mut here = BitSet::new();
    here.insert(name);
    let opened = press * (30 - time)
        + children
            .iter()
            .map(|(name, dist)| pressure(**name, time + *dist + 1, open.union(&here).collect()))
            .max()
            .unwrap_or(0);
    opened.max(walked)
}

fn to_usize(s: &str) -> usize {
    let mut s = s.chars();
    let a = s.next().unwrap() as usize - 64;
    let b = s.next().unwrap() as usize - 64;
    a * 100 + b
}
