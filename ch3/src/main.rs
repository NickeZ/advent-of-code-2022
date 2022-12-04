use std::io::Read;
use std::collections::HashSet;

fn solve(input: &str) -> i32 {
    let mut acc = 0;
    for line in input.trim().split('\n') {
        let res = find_character(line);
        acc += cvt(res);
    }
    acc
}

fn solve2(input: &str) -> i32 {
    let mut count = 0;
    let mut acc = 0;
    let mut elfs: Vec<HashSet<char>> = Vec::new();
    for line in input.trim().split('\n') {
        if count == 3 {
            acc += cvt(get_badge(&elfs));
            elfs.clear();
            elfs.push(line.chars().collect());
            count = 1;
        } else {
            elfs.push(line.chars().collect());
            count += 1;
        }
    }
    acc += cvt(get_badge(&elfs));
    acc
}

fn get_badge(elfs: &[HashSet<char>]) -> char {
    let tmp: HashSet<char> = elfs[0].intersection(&elfs[1]).cloned().collect();
    *tmp.intersection(&elfs[2]).next().unwrap()
}

// Lowercase item types a through z have priorities 1 through 26.
// Uppercase item types A through Z have priorities 27 through 52.
fn cvt(c: char) -> i32 {
    let b = c as u8;
    if b > b'Z' {
        b - b'a' + 1
    } else {
        b - b'A' + 27
    }.into()
}

fn find_character(input: &str) -> char {
    let first_half = &input[..input.len()/2];
    let second_half = &input[input.len()/2..];
    for c in first_half.chars() {
        for c2 in second_half.chars() {
            if c == c2 {
                return c
            }
        }
    }
    panic!()
}

fn main() {
    let mut file = std::fs::File::open("input").unwrap();
    let mut content = String::new();
    file.read_to_string(&mut content).unwrap();
    {
        let res = solve(&content);
        println!("{}", res);
    }
    {
        let res = solve2(&content);
        println!("{}", res);
    }
}

#[test]
fn test() {
    let input = "vJrwpWtwJgWrhcsFMMfFFhFp
jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
PmmdzqPrVvPwwTWBwg
wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
ttgJtRGJQctTZtZT
CrZsJsPPZsGzwwsLwLmpwMDw";
    assert_eq!(157, solve(&input));
    assert_eq!(70, solve2(&input));
}
