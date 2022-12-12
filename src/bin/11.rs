use itertools::Itertools;

#[derive(Clone)]
struct Monkey {
    items: Vec<u64>,
    op: Op,
    div: u64,
    true_false: (usize, usize),
    inspects: usize,
}

#[derive(Clone)]
enum Op {
    Add(u64),
    Mul(u64),
    Pow,
}

#[aoc::main(11)]
fn main(input: &str) -> (usize, usize) {
    let mut monkeys: Vec<Monkey> = input.split("\n\n").filter_map(|s| make_monkey(s)).collect();
    let modulo: u64 = monkeys.iter().map(|m| m.div).product();
    (
        biz(&mut monkeys.clone(), 20, |x| x / 3),
        biz(&mut monkeys, 10000, |x| x % modulo),
    )
}

fn biz(monkeys: &mut Vec<Monkey>, rounds: usize, func: impl Fn(u64) -> u64) -> usize {
    for _ in 0..rounds {
        round(monkeys, &func);
    }
    let sorted: Vec<usize> = monkeys.iter().map(|m| m.inspects).sorted().rev().collect();
    sorted[0] * sorted[1]
}

fn round(monkeys: &mut Vec<Monkey>, func: impl Fn(u64) -> u64) {
    for i in 0..monkeys.len() {
        let m = &mut monkeys[i];
        let (if_true, if_false) = m.true_false;
        let items = m.items.clone();
        let mut items = items.iter();
        m.items.clear();
        m.inspects += items.len();

        while let Some(worry) = items.next() {
            let new_worry = func(match monkeys[i].op {
                Op::Add(v) => worry + v,
                Op::Mul(v) => worry * v,
                Op::Pow => worry.pow(2),
            });
            if new_worry % monkeys[i].div == 0 {
                monkeys[if_true].items.push(new_worry);
            } else {
                monkeys[if_false].items.push(new_worry);
            }
        }
    }
}

fn make_monkey(s: &str) -> Option<Monkey> {
    let mut s = s.lines().skip(1);
    let items: Vec<u64> = s
        .next()?
        .replace(",", "")
        .split_whitespace()
        .filter_map(|w| w.parse().ok())
        .collect();
    let (a, sign, b) = s
        .next()?
        .split_once("= ")?
        .1
        .split_whitespace()
        .next_tuple()?;
    let op = if let Ok(val) = a.parse().or(b.parse()) {
        match sign {
            "+" => Op::Add(val),
            _ => Op::Mul(val),
        }
    } else {
        Op::Pow
    };
    Some(Monkey {
        items,
        op,
        div: find_int(s.next()?)? as u64,
        true_false: (find_int(s.next()?)?, find_int(s.next()?)?),
        inspects: 0,
    })
}

fn find_int(s: &str) -> Option<usize> {
    s.split_whitespace().find_map(|w| w.parse().ok())
}
