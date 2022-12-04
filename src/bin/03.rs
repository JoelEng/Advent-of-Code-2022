use array_tool::vec::Intersect;
use itertools::Itertools;

#[aoc::main(03)]
fn main(input: &str) -> (usize, usize) {
    let sacks: Vec<&[u8]> = input.lines().map(|l| l.as_bytes()).collect();
    (p1(&sacks), p2(&sacks))
}

fn p1(sacks: &Vec<&[u8]>) -> usize {
    let mut p1 = 0;
    for l in sacks {
        let (s1, s2) = l.split_at(l.len() / 2);
        let i = s1.to_vec().intersect(s2.to_vec());
        p1 += prio(i.get(0).unwrap());
    }
    p1
}

fn p2(sacks: &Vec<&[u8]>) -> usize {
    let mut p2 = 0;
    for (s1, s2, s3) in sacks.iter().tuples() {
        let i = s1.to_vec().intersect(s2.to_vec()).intersect(s3.to_vec());
        p2 += prio(i.get(0).unwrap());
    }
    p2
}

fn prio(i: &u8) -> usize {
    (97..=122).chain(65..=90).position(|e| e == *i).unwrap() + 1
}
