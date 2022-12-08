use std::str;

enum System {
    File(u32),
    Dir(Vec<System>),
}

#[aoc::main(07)]
fn main(input: &str) -> (u32, u32) {
    let mut cmds = input.lines().skip(1);
    let mut dirs = vec![];
    solve(parse_tree(&mut cmds), &mut dirs);
    let p1: u32 = dirs.iter().filter(|d| d < &&100000).sum();
    let space_to_free = dirs.iter().max().unwrap() - 40000000;
    let p2 = dirs.iter().filter(|d| d >= &&space_to_free).min().unwrap();
    (p1, *p2)
}

fn solve(dir: Vec<System>, dirs: &mut Vec<u32>) -> u32 {
    let mut size = 0;
    for s in dir {
        size += match s {
            System::File(s) => s,
            System::Dir(c) => solve(c, dirs),
        }
    }
    dirs.push(size);
    size
}

fn parse_tree<'a, T: Iterator<Item = &'a str>>(cmds: &mut T) -> Vec<System> {
    let mut v = vec![];
    while let Some(cmd) = cmds.next() {
        match cmd.split_whitespace().collect::<Vec<_>>().as_slice() {
            &[z, _] => {
                z.parse().and_then(|z| Ok(v.push(System::File(z)))).ok();
            }
            &[_, _, ".."] => break,
            _ => v.push(System::Dir(parse_tree(cmds))),
        }
    }
    v
}
