use bit_set::BitSet;
use hashbrown::HashMap;
use memoize::memoize;
use std::cell::RefCell;

std::thread_local! {
  static VALVES : RefCell<HashMap<usize, (u32, Vec<usize>)>> = RefCell::new(HashMap::new());
}

#[aoc::main(16)]
fn main(input: &str) -> (u32, i32) {
    let input = input.replace(&['=', ';', ','][..], " ");
    let valves: HashMap<usize, (u32, Vec<usize>)> = input
        .lines()
        .map(|l| {
            let mut v = l
                .split_whitespace()
                .filter(|w| w.chars().all(|c| c.is_uppercase()))
                .map(|s| to_usize(s));
            let n = l.split_whitespace().find_map(|w| w.parse().ok()).unwrap();
            (v.next().unwrap(), (n, v.collect()))
        })
        .collect();
    VALVES.with(|v| v.replace(valves));

    // p1 works on the actual problem, but not on the example input
    let p1 = pressure(to_usize("AA"), 1, BitSet::new());
    (p1, 0)
}

#[memoize]
fn pressure(name: usize, time: u32, open: BitSet) -> u32 {
    if time == 30 {
        return 0;
    }
    let v = VALVES
        .with(|valves| valves.borrow().get(&name).cloned())
        .unwrap();
    if time == 29 {
        return v.0;
    }
    let children: Vec<&usize> = v.1.iter().filter(|v| !open.contains(**v)).collect();
    let walked = children
        .iter()
        .map(|v| pressure(**v, time + 1, open.clone()))
        .max()
        .unwrap_or(0);

    if v.0 == 0 {
        return walked;
    }

    let mut here = BitSet::new();
    here.insert(name);
    let opened = v.0 * (30 - time)
        + children
            .iter()
            .map(|v| pressure(**v, time + 2, open.union(&here).collect()))
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
