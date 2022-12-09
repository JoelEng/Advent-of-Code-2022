use hashbrown::HashSet;

type Pos = (i32, i32);

#[aoc::main(09)]
fn main(input: &str) -> (usize, usize) {
    let motions: Vec<(char, i32)> = input
        .lines()
        .map(|l| l.split_once(" ").unwrap())
        .map(|(d, v)| (d.chars().next().unwrap(), v.parse().unwrap()))
        .collect();
    let mut set: HashSet<Pos> = HashSet::new();
    let mut long_set: HashSet<Pos> = HashSet::new();
    let mut rope = [(0, 0); 10];
    for (d, v) in motions {
        for _ in 0..v {
            move_rope(&mut rope, d);
            set.insert(rope[1]);
            long_set.insert(rope[9]);
        }
    }
    (set.len(), long_set.len())
}

fn move_rope(rope: &mut [Pos], d: char) {
    match d {
        'R' => rope[0].0 += 1,
        'L' => rope[0].0 -= 1,
        'U' => rope[0].1 += 1,
        'D' => rope[0].1 -= 1,
        _ => (),
    };
    for i in 1..rope.len() {
        rope[i] = move_tail(rope[i - 1], rope[i]);
    }
}

fn move_tail(head: Pos, tail: Pos) -> Pos {
    let diff_0 = head.0 - tail.0;
    let diff_1 = head.1 - tail.1;
    if diff_0.abs() > 1 || diff_1.abs() > 1 {
        return (tail.0 + diff_0.signum(), tail.1 + diff_1.signum());
    }
    tail
}
