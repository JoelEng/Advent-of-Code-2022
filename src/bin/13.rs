use itertools::Itertools;
use serde_json::Value;
use std::cmp::Ordering;

#[aoc::main(13)]
fn main(input: &str) -> (usize, usize) {
    let input = input.replace("\n\n", "\n");
    let lists = input.lines().map(|l| val(l));
    let mut p1 = 0;
    for (i, (l, r)) in lists.clone().tuples().enumerate() {
        if cmp(&l, &r).is_lt() {
            p1 += i + 1;
        }
    }
    let m: Vec<Value> = "[[2]]\n[[6]]".lines().map(|l| val(l)).collect();
    let sorted = lists.chain(m.clone()).sorted_by(|l, r| cmp(l, r));
    let mut p2 = 1;
    for (i, v) in sorted.enumerate() {
        if m.iter().any(|m| cmp(m, &v).is_eq()) {
            p2 *= i + 1;
        }
    }
    (p1, p2)
}

fn cmp(l: &Value, r: &Value) -> Ordering {
    match (l, r) {
        (Value::Number(l), Value::Number(r)) => l.as_u64().cmp(&r.as_u64()),
        (Value::Array(_), Value::Number(_)) => cmp(l, &Value::Array(vec![r.to_owned()])),
        (Value::Number(_), Value::Array(_)) => cmp(&Value::Array(vec![l.to_owned()]), r),
        (Value::Array(l), Value::Array(r)) => {
            match l.iter().zip(r).map(|(l, r)| cmp(l, r)).find(|o| !o.is_eq()) {
                Some(o) => o,
                None => l.len().cmp(&r.len()),
            }
        }
        _ => unreachable!(),
    }
}

fn val(s: &str) -> Value {
    serde_json::from_str::<Value>(s).unwrap()
}
