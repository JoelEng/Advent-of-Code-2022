#[aoc::main(02)]
fn main(input: &str) -> (i32, i32) {
    let pairs: Vec<(i32, i32)> = input
        .lines()
        .map(|l| {
            let mut c = l.chars().into_iter();
            (c.nth(0).unwrap() as i32 % 64, c.nth(1).unwrap() as i32 % 87)
        })
        .collect();

    (p1(&pairs), p2(&pairs))
}

fn p1(pairs: &Vec<(i32, i32)>) -> i32 {
    let mut score = 0;
    for (opp, res) in pairs {
        let diff = res - opp;
        score += res;
        if diff == 1 || diff == -2 {
            score += 6;
        } else if diff == 0 {
            score += 3;
        }
    }
    score
}

fn p2(pairs: &Vec<(i32, i32)>) -> i32 {
    let mut score = 0;
    for (opp, end) in pairs {
        let res = match end {
            1 => opp - 1,
            2 => {
                score += 3;
                *opp
            }
            3 => {
                score += 6;
                opp + 1
            }
            _ => unreachable!(),
        };
        score += if res == 0 {
            3
        } else if res == 4 {
            1
        } else {
            res
        }
    }
    score
}
