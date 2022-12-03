use array_tool::vec::Intersect;

#[aoc::main(03)]
fn main(input: &str) -> (usize, usize) {
    let sacks: Vec<&[u8]> = input.lines().map(|l| l.as_bytes()).collect();
    (p1(&sacks), p2(&sacks).unwrap())
}

fn p1(sacks: &Vec<&[u8]>) -> usize {
    let sacks: Vec<(Vec<u8>, Vec<u8>)> = sacks
        .iter()
        .map(|l| (l[0..l.len() / 2].to_vec(), l[l.len() / 2..].to_vec()))
        .collect();
    let mut p1 = 0;
    for (s1, s2) in sacks {
        let i = s1.intersect(s2);
        p1 += prio(i.get(0).unwrap());
    }
    p1
}

fn p2(sacks: &Vec<&[u8]>) -> Option<usize> {
    let mut p2 = 0;
    for three in sacks.chunks(3) {
        let i = three
            .get(0)?
            .to_vec()
            .intersect(three.get(1)?.to_vec())
            .intersect(three.get(2)?.to_vec());
        p2 += prio(i.get(0)?);
    }
    Some(p2)
}

fn prio(i: &u8) -> usize {
    (97..=122).chain(65..=90).position(|e| e == *i).unwrap() + 1
}
