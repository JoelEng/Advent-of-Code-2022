use itertools::Itertools;
use regex::Regex;
use std::cmp::Ordering;

enum Packet {
    Int(u32),
    List(Vec<Packet>),
}

#[aoc::main(13)]
fn main(input: &str) -> (usize, usize) {
    let r = Regex::new(r"\d+|\[|\]").unwrap();
    let pairs = input.split("\n\n").filter_map(|p| {
        p.lines()
            .map(|l| get_list(&mut r.find_iter(l).map(|m| m.as_str()).skip(1)))
            .next_tuple()
    });
    let mut p1 = 0;
    for (i, (l, r)) in pairs.enumerate() {
        if in_order(&l, &r).is_lt() {
            p1 += i + 1;
        }
    }

    let pairs2 = (input.replace("\n\n", "\n") + "\n[[2]]\n[[6]]")
        .lines()
        .map(|l| get_list(&mut r.find_iter(l).map(|m| m.as_str()).skip(1)))
        .sorted_by(|l, r| in_order(l, r));
    let m: Vec<Vec<Packet>> = "[[2]]\n[[6]]"
        .lines()
        .map(|l| get_list(&mut r.find_iter(l).map(|m| m.as_str())))
        .collect();
    let mut p2 = 1;
    for (i, v) in pairs2.enumerate() {
        if m.iter().any(|m| in_order(m, &v).is_eq()) {
            p2 *= i + 1;
        }
    }

    (p1, p2)
}

fn get_list<'a, T: Iterator<Item = &'a str>>(packets: &mut T) -> Vec<Packet> {
    let mut v = vec![];
    while let Some(val) = packets.next() {
        if val == "]" {
            break;
        } else if val == "[" {
            v.push(Packet::List(get_list(packets)));
        } else if let Ok(num) = val.parse() {
            v.push(Packet::Int(num));
        }
    }
    v
}

fn in_order(l: &Vec<Packet>, r: &Vec<Packet>) -> Ordering {
    for (l, r) in l.iter().zip(r) {
        let order = match (l, r) {
            (Packet::Int(l), Packet::Int(r)) => l.cmp(r),
            (Packet::List(l), Packet::List(r)) => in_order(l, r),
            (Packet::List(l), Packet::Int(r)) => in_order(l, &vec![Packet::Int(*r)]),
            (Packet::Int(l), Packet::List(r)) => in_order(&vec![Packet::Int(*l)], r),
        };
        if !order.is_eq() {
            return order;
        }
    }
    l.len().cmp(&r.len())
}
