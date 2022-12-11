#[aoc::main(10)]
fn main(input: &str) -> (i32, &str) {
    let instrs = input.split_whitespace();

    let mut x = 1;
    let mut cycles = vec![0];
    for (i, instr) in instrs.enumerate() {
        cycles.push(x);
        draw(x, i);
        if let Ok(n) = instr.parse::<i32>() {
            x += n;
        }
    }
    println!("");
    let p1 = [20, 60, 100, 140, 180, 220]
        .into_iter()
        .map(|c| c * cycles[c as usize])
        .sum();
    (p1, "EPJBRKAH")
}

fn draw(x: i32, i: usize) {
    if i % 40 == 0 {
        println!("");
    }
    if (x - i as i32 % 40).abs() <= 1 {
        print!("█");
    } else {
        print!(" ");
    }
}
