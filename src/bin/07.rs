use std::str;

enum System {
    File(String, u32),
    Dir(String, Vec<System>),
}

#[aoc::main(07)]
fn main(input: &str) -> (u32, u32) {
    let mut cmds = input.split("$").skip(2);
    let home = execute(&mut cmds);
    // print_dir(&home, "  ");
    let (p1_ans, size) = p1(&home);
    let p2_ans = p2(size - 40000000, &home).0;
    let v = p2_ans.iter().min().unwrap();
    (p1_ans, v.to_owned())
}

fn p1(dir: &Vec<System>) -> (u32, u32) {
    let mut size = 0;
    let mut tot = 0;
    for s in dir {
        match s {
            System::File(n, s) => size += s,
            System::Dir(n, c) => {
                let (t, s) = p1(c);
                tot += t;
                size += s;
            }
        }
    }
    if size <= 100000 {
        tot += size;
    }
    (tot, size)
}

fn p2(space_to_free: u32, dir: &Vec<System>) -> (Vec<u32>, u32) {
    let mut size = 0;
    let mut big_dirs = vec![];
    for s in dir {
        match s {
            System::File(n, s) => size += s,
            System::Dir(n, c) => {
                let (mut t, s) = p2(space_to_free, c);
                big_dirs.append(&mut t);
                size += s;
            }
        }
    }
    if size > space_to_free {
        big_dirs.push(size);
    }
    (big_dirs, size)
}

fn execute<'a, T: Iterator<Item = &'a str>>(cmds: &mut T) -> Vec<System> {
    let mut v: Vec<System> = vec![];
    while let Some(cmd) = cmds.next() {
        let mut cmd = cmd.trim().lines();
        let mut a = cmd.next().unwrap().split_whitespace();
        let args_count = a.clone().count();
        match args_count {
            2 => {
                let name = a.nth(1).unwrap();
                if name == ".." {
                    break;
                }
                v.push(System::Dir(name.to_string(), execute(cmds)));
            }
            1 => {
                for f in cmd {
                    let (size, name) = f.split_once(" ").unwrap();
                    if size.parse::<u32>().is_ok() {
                        v.push(System::File(name.to_string(), size.parse().unwrap()));
                    }
                }
            }
            _ => unreachable!(),
        };
    }
    v
}
