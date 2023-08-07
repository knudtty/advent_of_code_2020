fn determine_valid_password(line: &str) -> bool {
    let mut min = String::new();
    let mut max = String::new();
    let c: char;
    let mut iter = line.chars().peekable();
    // min
    while iter.peek().unwrap().is_digit(10) {
        min.push(iter.next().unwrap());
    }
    // kill '-'
    iter.next();
    // max
    while iter.peek().unwrap().is_digit(10) {
        max.push(iter.next().unwrap());
    }
    // kill space
    iter.next();
    // get char
    c = iter.next().unwrap();
    // kill ':'
    iter.next();
    // get password
    let mut occurrances: usize = 0;
    while let Some(v) = iter.next() {
        if v == c {
            occurrances += 1;
        }
    }
    return occurrances >= min.parse::<usize>().unwrap()
        && occurrances <= max.parse::<usize>().unwrap();
}

fn determine_valid_password2(line: &str) -> bool {
    let mut n1 = String::new();
    let mut n2 = String::new();
    let c: char;
    let mut iter = line.chars().peekable();
    // min
    while iter.peek().unwrap().is_digit(10) {
        n1.push(iter.next().unwrap());
    }
    let n1: usize = n1.parse::<usize>().unwrap()-1;
    // kill '-'
    iter.next();
    // max
    while iter.peek().unwrap().is_digit(10) {
        n2.push(iter.next().unwrap());
    }
    let n2: usize = n2.parse::<usize>().unwrap()-1;
    // kill space
    iter.next();
    // get char
    c = iter.next().unwrap();
    // kill ':'
    iter.next();
    // kill ' '
    iter.next();
    let c1 = iter.clone().nth(n1).unwrap();
    match iter.clone().nth(n2) {
        Some(c2) => return (c1 == c || c2 == c) && c1 != c2,
        None => return true,
    }
}

fn main() {
    let contents = include_str!("../../inputs/day2.txt");
    println!(
        "Answer 1: {}",
        contents
            .lines()
            .filter(|line| determine_valid_password(line))
            .count()
    );
    println!(
        "Answer 2: {}",
        contents
            .lines()
            .filter(|line| determine_valid_password2(line))
            .count()
    );
}
