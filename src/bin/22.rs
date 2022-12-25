use itertools::Itertools;
use memoize::memoize;

const DIRS: [(i64, i64); 4] = [(1, 0), (0, 1), (-1, 0), (0, -1)];
type Pos = (i64, i64);

#[derive(Debug)]
enum Instr {
    Num(u32),
    Letter(char),
}

#[aoc::main(22)]
fn main(input: &str) -> (i64, i64) {
    let (chars, instrs) = input.split("\n\n").collect_tuple().unwrap();
    let instrs = get_instrs(instrs);
    let mut chars: Vec<Vec<char>> = chars.lines().map(|l| l.chars().collect()).collect();
    let row_len = chars
        .len()
        .max(chars.iter().map(|r| r.len()).max().unwrap());
    for r in &mut chars {
        r.resize(row_len, ' ');
    }
    let p1 = solve(&chars, &instrs, false);
    println!("Starting part 2...");
    let p2 = solve(&chars, &instrs, true);

    (p1, p2)
}

fn solve(chars: &Vec<Vec<char>>, instrs: &Vec<Instr>, p2: bool) -> i64 {
    let mut p: Pos = (chars[0].iter().position(|c| *c != ' ').unwrap() as i64, 0);
    // println!("init pos: {:?}", p);
    let mut dir = (1, 0);
    for instr in instrs {
        match instr {
            Instr::Num(n) => {
                for _ in 0..*n {
                    if let Some(a) = pos(p, dir, chars.clone(), p2) {
                        (p, dir) = a;
                        // println!("p {:?}", p);
                    } else {
                        break;
                    }
                }
                // println!("pos after {:?}: {:?}", instr, p);
                // println!("dir: {:?}", dir);
            }
            Instr::Letter(l) => {
                let i = DIRS.iter().position(|p| *p == dir).unwrap() as i64;
                dir = DIRS[match l {
                    'L' => (i - 1).rem_euclid(DIRS.len() as i64),
                    'R' => (i + 1).rem_euclid(DIRS.len() as i64),
                    _ => unreachable!(),
                } as usize];
                // println!("dir after {:?}: {:?}", instr, dir);
            }
        }
    }
    (p.1 + 1) * 1000 + (p.0 + 1) * 4 + DIRS.iter().position(|p| *p == dir).unwrap() as i64
}

//  1 2
//  3
//4 5
//6

#[memoize]
fn pos(p: Pos, dir: Pos, chars: Vec<Vec<char>>, p2: bool) -> Option<(Pos, Pos)> {
    let mut dir = dir;
    let (xlen, ylen) = (chars[0].len() as i64, chars.len() as i64);
    let mut new = (p.0 + dir.0, p.1 + dir.1);
    if p2 {
        if new.0 < 0 || new.1 < 0 || new.0 >= xlen || new.1 >= ylen {
            let (xlast, ylast) = (xlen - 1, ylen - 1);
            let (xoffset, yoffset) = (p.0 % 50, p.1 % 50);
            (new, dir) = match (p.0 / 50, p.1 / 50, dir) {
                (0, 0, (-1, 0)) => ((0, 3 * 50 - 1 - yoffset), (1, 0)),
                (0, 0, (0, -1)) => ((0, 1 * 50 + xoffset), (1, 0)),
                (1, 0, _) => ((0, 3 * 50 + xoffset), (1, 0)),
                (_, 0, (1, 0)) => ((xlast, 3 * 50 - 1 - yoffset), (-1, 0)),
                (_, 0, (0, -1)) => ((xoffset, ylast), (0, -1)),
                (0, 1, _) => ((yoffset, 0), (0, 1)),
                (_, 1, _) => ((2 * 50 + yoffset, ylast), (0, -1)),
                (0, 2, _) => ((0, 1 * 50 - 1 - yoffset), (1, 0)),
                (_, 2, _) => ((xlast, 1 * 50 - 1 - yoffset), (-1, 0)),
                (0, 3, (-1, 0)) => ((1 * 50 + yoffset, 0), (0, 1)),
                (0, 3, (0, 1)) => ((2 * 50 + xoffset, 0), (0, 1)),
                (1, 3, _) => ((xlast, 3 * 50 + xoffset), (-1, 0)),
                (_, 3, (1, 0)) => ((1 * 50 + yoffset, ylast), (0, -1)),
                (_, 3, (0, 1)) => ((xlast, 1 * 50 + xoffset), (-1, 0)),
                _ => unreachable!(),
            }
        }
    } else {
        new = (new.0.rem_euclid(xlen), new.1.rem_euclid(ylen));
    }
    match chars[new.1 as usize][new.0 as usize] {
        ' ' => pos(new, dir, chars.clone(), p2),
        '#' => None,
        '.' => Some((new, dir)),
        _ => unreachable!(),
    }
}

fn get_instrs(s: &str) -> Vec<Instr> {
    let mut instrs = vec![];
    let mut num = 0;
    for c in s.chars() {
        if let Some(n) = c.to_digit(10) {
            num *= 10;
            num += n;
        } else {
            instrs.push(Instr::Num(num));
            num = 0;
            instrs.push(Instr::Letter(c));
        }
    }
    instrs
}
