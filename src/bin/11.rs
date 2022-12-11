use itertools::Itertools;

#[derive(Clone)]
struct Monkey {
    items: Vec<u64>,
    op: String,
    div: u64,
    if_true: usize,
    if_false: usize,
    inspects: usize,
}

#[aoc::main(11)]
fn main(input: &str) -> (usize, usize) {
    let mut monkeys: Vec<Monkey> = input
        .split("\n\n")
        .map(|s| Monkey::from(s).unwrap())
        .collect();
    let tot_div: u64 = monkeys.iter().map(|m| m.div).product();
    (
        biz(&mut monkeys.clone(), true, 20, tot_div),
        biz(&mut monkeys, false, 10000, tot_div),
    )
}

fn biz(monkeys: &mut Vec<Monkey>, p1: bool, rounds: usize, tot_div: u64) -> usize {
    for _ in 0..rounds {
        round(monkeys, p1, tot_div);
    }
    let sorted: Vec<usize> = monkeys.iter().map(|m| m.inspects).sorted().rev().collect();
    sorted[0] * sorted[1]
}

fn round(monkeys: &mut Vec<Monkey>, p1: bool, tot_div: u64) {
    for i in 0..monkeys.len() {
        let m = &mut monkeys[i];
        let (if_true, if_false) = (m.if_true, m.if_false);
        let div = m.div;
        let items = m.items.clone();
        let mut items = items.iter();
        m.items.clear();
        m.inspects += items.len();
        let op = m.op.to_owned();
        let (a, op, b) = op.split_whitespace().next_tuple().unwrap();

        while let Some(worry) = items.next() {
            let a: u64 = a.parse().unwrap_or(*worry);
            let b: u64 = b.parse().unwrap_or(*worry);
            let mut new_worry = if op == "+" { a + b } else { a * b };
            if p1 {
                new_worry /= 3;
            } else {
                new_worry %= tot_div;
            }
            if new_worry % div == 0 {
                monkeys[if_true].items.push(new_worry);
            } else {
                monkeys[if_false].items.push(new_worry);
            }
        }
    }
}

impl Monkey {
    fn from(s: &str) -> Option<Monkey> {
        let mut s = s.lines();
        s.next();
        let items: Vec<u64> = s
            .next()?
            .replace(",", "")
            .split_whitespace()
            .filter_map(|w| w.parse().ok())
            .collect();
        let op = s.next()?.split_once("= ")?.1.to_string();
        let div: u64 = s.next()?.split_whitespace().find_map(|w| w.parse().ok())?;
        let if_true = s.next()?.split_whitespace().find_map(|w| w.parse().ok())?;
        let if_false = s.next()?.split_whitespace().find_map(|w| w.parse().ok())?;

        Some(Monkey {
            items,
            op,
            div,
            if_true,
            if_false,
            inspects: 0,
        })
    }
}
