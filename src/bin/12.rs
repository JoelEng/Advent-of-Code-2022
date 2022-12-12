use hashbrown::HashMap;
use std::collections::VecDeque;

#[aoc::main(12)]
fn main(input: &str) -> (i32, i32) {
    let map: HashMap<(i32, i32), (char, i32)> = input
        .lines()
        .enumerate()
        .flat_map(|(y, l)| {
            l.chars()
                .enumerate()
                .map(move |(x, c)| ((x as i32, y as i32), (c, -1)))
        })
        .collect();
    (bfs(map.clone(), &['S']), bfs(map, &['S', 'a']))
}

fn bfs(mut map: HashMap<(i32, i32), (char, i32)>, start: &[char]) -> i32 {
    let mut q = VecDeque::new();
    let mut end = (0, 0);
    for (k, (c, v)) in &mut map {
        if start.contains(c) {
            *c = 'a';
            *v = 0;
            q.push_back(*k);
        } else if *c == 'E' {
            *c = 'z';
            end = *k;
        }
    }
    while let Some(k) = q.pop_front() {
        if k == end {
            return map.get(&k).unwrap().1;
        }
        let (c, d) = *map.get(&k).unwrap();
        for (x, y) in [(1, 0), (0, 1), (-1, 0), (0, -1)] {
            if let Some((k, v)) = map.get_key_value_mut(&(k.0 + x, k.1 + y)) {
                if v.1 == -1 && v.0 as i32 - c as i32 <= 1 {
                    v.1 = d + 1;
                    q.push_back(*k);
                }
            }
        }
    }
    0
}
