use std::io::Read;

// A for Rock, B for Paper, and C for Scissors
//
// X for Rock, Y for Paper, and Z for Scissors
//
// shape you selected (1 for Rock, 2 for Paper, and 3 for Scissors)
//
// outcome of the round (0 if you lost, 3 if the round was a draw, and 6 if you won).

fn solve(input: &str) -> i32 {
    let mut acc = 0;
    for line in input.trim().split('\n') {
        let mut split = line.split(' ');
        let opponent = split.next().unwrap();
        let me = split.next().unwrap();
        let point_from_shape = match me {
            "X" => 1,
            "Y" => 2,
            "Z" => 3,
            _ => panic!(),
        };
        let outcome = match (opponent, me) {
            ("A", "X") => 3,
            ("A", "Y") => 6,
            ("A", "Z") => 0,
            ("B", "X") => 0,
            ("B", "Y") => 3,
            ("B", "Z") => 6,
            ("C", "X") => 6,
            ("C", "Y") => 0,
            ("C", "Z") => 3,
            _ => panic!(),
        };
        let points = point_from_shape + outcome;
        acc += points;
    }
    acc
}

// A for Rock, B for Paper, and C for Scissors
// X means you need to lose, Y means you need to end the round in a draw, and Z means you need to win
fn solve2(input: &str) -> i32 {
    let mut acc = 0;
    for line in input.trim().split('\n') {
        let mut split = line.split(' ');
        let opponent = split.next().unwrap();
        let outcome = split.next().unwrap();
        let my_action = match outcome {
            "X" => match opponent {
                "A" => "C",
                "B" => "A",
                "C" => "B",
                _ => panic!(),
            },
            "Y" => opponent,
            "Z" => match opponent {
                "A" => "B",
                "B" => "C",
                "C" => "A",
                _ => panic!(),
            },
            _ => panic!(),
        };
        let point_from_shape = match my_action {
            "A" => 1,
            "B" => 2,
            "C" => 3,
            _ => panic!(),
        };
        let outcome_point = match outcome {
            "X" => 0,
            "Y" => 3,
            "Z" => 6,
            _ => panic!(),
        };
        let points = point_from_shape + outcome_point;
        acc += points;
    }
    acc
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
    let input = "A Y\nB X\nC Z";
    assert_eq!(15, solve(&input));
    assert_eq!(12, solve2(&input));
}
