use std::io::Read;
use std::ops::RangeInclusive;

fn solve(input: &str) -> i32 {
    let mut acc = 0;
    for line in input.trim().split('\n') {
        let mut line = line.split(',');
        let left = parse_range(line.next().unwrap());
        let right = parse_range(line.next().unwrap());

        if left.start() >= right.start() && left.end() <= right.end() {
            acc += 1;
        }
        else if right.start() >= left.start() && right.end() <= left.end() {
            acc += 1;
        }
    }
    acc
}

fn solve2(input: &str) -> i32 {
    let mut acc = 0;
    for line in input.trim().split('\n') {
        let mut line = line.split(',');
        let left = parse_range(line.next().unwrap());
        let right = parse_range(line.next().unwrap());

        // L: |---|
        // R:   |---|
        if right.start() <= left.end() && right.end() > left.end() {
            acc += 1;
        }
        // L:   |---|
        // R: |---|
        else if left.start() <= right.end() && left.end() > right.end() {
            acc += 1;
        }
        // L:    |---|
        // R: |---------|
        else if left.start() >= right.start() && left.end() <= right.end() {
            acc += 1;
        }
        // L: |---------|
        // R:    |---|
        else if right.start() >= left.start() && right.end() <= left.end() {
            acc += 1;
        }

    }
    acc
}

fn parse_range(input: &str) -> RangeInclusive<i32> {
    let mut input = input.split('-');
    str::parse::<i32>(input.next().unwrap()).unwrap()..=str::parse::<i32>(input.next().unwrap()).unwrap()
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
    let input = "2-4,6-8
2-3,4-5
5-7,7-9
2-8,3-7
6-6,4-6
2-6,4-8
";
    assert_eq!(2, solve(&input));
    assert_eq!(4, solve2(&input));
}
