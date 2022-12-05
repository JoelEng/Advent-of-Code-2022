use hashbrown::HashMap;
use itertools::Itertools;

#[aoc::main(05)]
fn main(input: &str) -> (String, String) {
    let (stacks, moves) = input.split_once("\n\n").unwrap();
    let stacks = get_stacks(stacks);
    let moves = get_moves(moves);

    (p1(stacks.clone(), &moves), p2(stacks.clone(), &moves))
}

fn p1(mut stacks: HashMap<usize, Vec<char>>, moves: &Vec<(usize, usize, usize)>) -> String {
    for (mv, a, b) in moves {
        for _ in 0..*mv {
            let pop = stacks.get_mut(a).unwrap().pop().unwrap();
            stacks.get_mut(b).unwrap().push(pop);
        }
    }

    let mut p1 = String::from("            ");
    for (i, s) in stacks {
        p1.insert(i, *s.last().unwrap());
    } // VCTFTJQCG, gives wrong answer
    p1.replace(" ", "")
}

fn p2(mut stacks: HashMap<usize, Vec<char>>, moves: &Vec<(usize, usize, usize)>) -> String {
    for (mv, a, b) in moves {
        let mut temp: Vec<char> = vec![];
        for _ in 0..*mv {
            let pop = stacks.get_mut(a).unwrap().pop().unwrap();
            temp.push(pop);
        }
        for _ in 0..*mv {
            stacks.get_mut(b).unwrap().push(temp.pop().unwrap());
        }
    }

    let mut p2 = String::from("            ");
    for (i, s) in stacks {
        p2.insert(i, *s.last().unwrap());
    } // GCFGLDNJZ, gives wrong answer
    p2.replace(" ", "")
}

fn get_stacks(stacks: &str) -> HashMap<usize, Vec<char>> {
    let stacks = stacks
        .replace("    ", "*")
        .replace(" ", "")
        .replace("[", "")
        .replace("]", "");
    let mut s: HashMap<usize, Vec<char>> = HashMap::new();
    let mut stacks = stacks.lines().rev();
    for a in stacks.next().unwrap().chars().filter(|a| a.is_numeric()) {
        s.insert(a.to_digit(10).unwrap() as usize, vec![]);
    }
    for l in stacks {
        let mut i = 1;
        for c in l.chars() {
            if c.is_alphabetic() {
                s.get_mut(&i).unwrap().push(c);
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
                .next_tuple()
                .unwrap()
        })
        .collect()
}
