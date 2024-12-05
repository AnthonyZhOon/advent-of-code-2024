const EXAMPLE: &str = "7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9
";

const TEST: &str = "60 59 56 55 54 52 51";
const INPUT: &str = include_str!("input.txt");
fn main() {
    let lines = INPUT.trim().split("\n");
    let result = lines
        .map(|row| {
            let x = row
                .split_ascii_whitespace()
                .map(|x| x.parse::<i32>().unwrap())
                .collect::<Vec<_>>();
            check(&x) || part2_brute(&x)
        })
        .filter(|x| *x)
        .count();
    println!("{result}");
}

fn check(line: &[i32]) -> bool {
    let x = line
        .windows(2)
        .map(|elems| {
            let [l, r] = elems.try_into().unwrap();
            r - l
        })
        .collect::<Vec<i32>>();
    x.windows(2).all(|slice| {
        let [l, r] = slice.try_into().unwrap();
        // println!("[{l}, {r}]");
        (1..=3).contains(&l.abs()) && (1..=3).contains(&r.abs()) && (l.cmp(&0) == r.cmp(&0))
    })
}

#[allow(unused)]
fn part2_brute(line: &[i32]) -> bool {
    let mut set = vec![true; line.len()];
    for i in 0..set.len() {
        set[i] = false;
        let arr = set
            .iter()
            .zip(line)
            .filter(|(p, _)| **p)
            .map(|(_, x)| *x)
            .collect::<Vec<_>>();
        if check(&arr) {
            return true;
        }
        set[i] = true;
    }
    false
}

#[allow(unused)]
fn part1(input: &str) -> u64 {
    let lines = input.trim().split("\n");
    let result = lines
        .map(|row| {
            let x = row
                .split_ascii_whitespace()
                .map(|x| x.parse::<i32>().unwrap())
                .collect::<Vec<_>>();
            check(&x)
        })
        .filter(|x| *x)
        .count();
    result as u64
}
