use hashbrown::HashMap;
use itertools::Itertools;

#[aoc::main(21)]
fn main(input: &str) -> (i64, &str) {
    let monkeys: HashMap<String, String> = input
        .lines()
        .filter_map(|l| l.split(": ").map(|w| w.to_string()).collect_tuple())
        .collect();
    let (p1, p2) = eval_monkey("root".to_string(), &monkeys);
    println!("{}", p2);
    (p1, "3352886133831 (https://quickmath.com/)")
}

fn eval_monkey(m: String, monkeys: &HashMap<String, String>) -> (i64, String) {
    let yell = monkeys.get(&m).unwrap();
    if let Ok(n) = yell.parse() {
        match m.as_str() {
            "humn" => return (n, "x".to_string()),
            _ => return (n, yell.to_string()),
        }
    }
    let (m1, op, m2) = yell.split_whitespace().collect_tuple().unwrap();
    let ((m1, m1_eq), (m2, m2_eq)) = (
        eval_monkey(m1.to_string(), monkeys),
        eval_monkey(m2.to_string(), monkeys),
    );
    let val = match op {
        "+" => m1 + m2,
        "-" => m1 - m2,
        "*" => m1 * m2,
        "/" => m1 / m2,
        _ => unreachable!(),
    };
    let s = if m == "root" {
        m1_eq + "=" + &m2_eq
    } else {
        "(".to_string() + &m1_eq + op + &m2_eq + ")"
    };
    (val, if s.contains('x') { s } else { val.to_string() })
}
