use std::io::Read;
use std::cmp::Ordering;

fn main() {
    let mut file = std::fs::File::open("input").unwrap();
    let mut content = String::new();
    file.read_to_string(&mut content).unwrap();
    {
        let mut acc = 0;
        let mut largest = 0;
        for line in content.split('\n') {
            if line == "" {
                if acc > largest {
                    largest = acc;
                }
                acc = 0;
            } else {
                acc += str::parse::<i32>(line).unwrap();
            }
        }
        println!("{}", largest);
    }
    {
        let mut acc = 0;
        let mut all = Vec::new();
        for line in content.split('\n') {
            if line == "" {
                all.push(acc);
                acc = 0;
            } else {
                acc += str::parse::<i32>(line).unwrap();
            }
        }
        all.sort_by(|a, b| if b > a { Ordering::Greater } else { Ordering::Less });
        println!("{}", &all[0..3].iter().sum::<i32>());
    }
}
