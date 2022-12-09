#[aoc::main(08)]
fn main(input: &str) -> (u32, u32) {
    let grid: Vec<Vec<u32>> = input
        .lines()
        .map(|l| l.chars().filter_map(|c| c.to_digit(10)).collect())
        .collect();

    let mut p1 = 0;
    let mut p2 = 0;
    for (x, row) in grid.iter().enumerate() {
        for (y, t) in row.iter().enumerate() {
            let mut col: Vec<u32> = grid.iter().map(|r| r[y]).collect();
            let mut row_copy = row.clone();
            let ((w, e), (n, s)) = (row_copy.split_at_mut(y), col.split_at_mut(x));
            w.reverse();
            n.reverse();
            let (e, s) = (&e[1..], &s[1..]);

            let seen = [w, e, n, s].iter().any(|d| d.iter().all(|o| o < t));
            p1 += seen as u32;

            let scenic: u32 = [w, e, n, s].iter().map(|d| visible_trees(d, t)).product();
            if scenic > p2 {
                p2 = scenic;
            }
        }
    }
    (p1, p2)
}

fn visible_trees(d: &[u32], t: &u32) -> u32 {
    let mut count = 0;
    for o in d {
        count += 1;
        if o >= t {
            break;
        }
    }
    count
}
