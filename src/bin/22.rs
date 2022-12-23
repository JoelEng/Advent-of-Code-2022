use itertools::Itertools;
use memoize::memoize;

const DIRS: [(i32, i32); 4] = [(1, 0), (0, 1), (-1, 0), (0, -1)];
type Pos = (i32, i32);

#[derive(Debug)]
enum Instr {
    Num(u32),
    Letter(char),
}

#[aoc::main(22)]
fn main(input: &str) -> (i32, i32) {
    let (chars, instrs) = input.split("\n\n").collect_tuple().unwrap();
    let instrs = get_instrs(instrs);
    let mut chars: Vec<Vec<char>> = chars.lines().map(|l| l.chars().collect()).collect();
    let row_len = chars
        .len()
        .max(chars.iter().map(|r| r.len()).max().unwrap());
    for r in &mut chars {
        r.resize(row_len, ' ');
    }

    let mut p: Pos = (chars[0].iter().position(|c| *c != ' ').unwrap() as i32, 0);
    // println!("init pos: {:?}", p);
    let mut dir = (1, 0);
    for instr in instrs {
        match instr {
            Instr::Num(n) => {
                for _ in 0..n {
                    if let Some(a) = pos(p, dir, chars.clone()) {
                        p = a;
                        // println!("p {:?}", p);
                    } else {
                        break;
                    }
                }
                // println!("pos after {:?}: {:?}", instr, p);
            }
            Instr::Letter(l) => {
                let i = DIRS.iter().position(|p| *p == dir).unwrap() as i32;
                dir = DIRS[match l {
                    'L' => (i - 1).rem_euclid(DIRS.len() as i32),
                    'R' => (i + 1).rem_euclid(DIRS.len() as i32),
                    _ => unreachable!(),
                } as usize];
                // println!("dir after {:?}: {:?}", instr, dir);
            }
        }
    }
    let p1 = (p.1 + 1) * 1000 + (p.0 + 1) * 4 + DIRS.iter().position(|p| *p == dir).unwrap() as i32;

    (p1, 0)
}

#[memoize]
fn pos(p: Pos, dir: Pos, chars: Vec<Vec<char>>) -> Option<Pos> {
    let len = chars.len() as i32;
    let new = ((p.0 + dir.0).rem_euclid(len), (p.1 + dir.1).rem_euclid(len));
    // println!("new: {:?}", new);
    match chars[new.1 as usize][new.0 as usize] {
        ' ' => pos(new, dir, chars.clone()),
        '#' => None,
        '.' => Some(new),
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
