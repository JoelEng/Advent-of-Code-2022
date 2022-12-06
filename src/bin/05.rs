use itertools::Itertools;
use std::str;

#[aoc::main(05)]
fn main(input: &str) -> (String, String) {
    let (stacks, moves) = input.split_once("\n\n").unwrap();
    let (stacks, moves) = (get_stacks(stacks), get_moves(moves));
    (p1(stacks.clone(), &moves), p2(stacks, &moves))
}

fn p1(mut stacks: Vec<Vec<char>>, moves: &Vec<(usize, usize, usize)>) -> String {
    for (mv, a, b) in moves {
        for _ in 0..*mv {
            let pop = stacks[a - 1].pop().unwrap();
            stacks[b - 1].push(pop);
        }
    }
    stacks_to_str(stacks)
}

fn p2(mut stacks: Vec<Vec<char>>, moves: &Vec<(usize, usize, usize)>) -> String {
    for (mv, a, b) in moves {
        let mut temp: Vec<char> = vec![];
        for _ in 0..*mv {
            let pop = stacks[a - 1].pop().unwrap();
            temp.push(pop);
        }
        for _ in 0..*mv {
            stacks[b - 1].push(temp.pop().unwrap());
        }
    }
    stacks_to_str(stacks)
}

fn stacks_to_str(stacks: Vec<Vec<char>>) -> String {
    stacks.iter().map(|v| v.last().unwrap()).collect()
}

fn get_stacks(stacks: &str) -> Vec<Vec<char>> {
    let stacks = stacks
        .replace("    ", "*")
        .replace(" ", "")
        .replace("[", "")
        .replace("]", "");
    let mut s: Vec<Vec<char>> = vec![vec![]; 9];
    let stacks = stacks.lines().rev();
    for l in stacks {
        let mut i = 0;
        for c in l.chars() {
            if c.is_alphabetic() {
                s[i].push(c);
            }
            i += 1;
        }
    }
    s
}

fn get_moves(moves: &str) -> Vec<(usize, usize, usize)> {
    moves
        .lines()
        .map(|l| {
            l.split_whitespace()
                .filter_map(|w| w.parse().ok())
                .collect_tuple()
                .unwrap()
        })
        .collect()
}
