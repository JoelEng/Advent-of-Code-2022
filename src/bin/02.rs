#[aoc::main(02)]
fn main(input: &str) -> (i32, i32) {
    let pairs: Vec<(i32, i32)> = input
        .lines()
        .map(|l| (l.as_bytes()[0] as i32 - 64, l.as_bytes()[2] as i32 - 87))
        .collect();

    let p1: i32 = pairs.iter().map(|(o, r)| calc_score(o, r)).sum();
    (p1, p2(&pairs))
}

fn p2(pairs: &Vec<(i32, i32)>) -> i32 {
    let mut score = 0;
    for (opp, end) in pairs {
        let res = match end {
            1 => opp - 1,
            2 => *opp,
            3 => opp + 1,
            _ => unreachable!(),
        };
        score += calc_score(opp, &((res - 1).rem_euclid(3) + 1));
    }
    score
}

fn calc_score(opp: &i32, res: &i32) -> i32 {
    let diff = res - opp;
    res + if diff == 1 || diff == -2 {
        6
    } else if diff == 0 {
        3
    } else {
        0
    }
}
