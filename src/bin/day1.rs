use std::{cell::RefCell, collections::HashSet, rc::Rc};

fn main() {
    let contents = include_str!("../../inputs/day1.txt");
    let sum_num = 2020;
    let mut answer = 0;
    let mut set: HashSet<usize> = HashSet::new();
    for num in contents.lines().map(|str| str.parse::<usize>().unwrap()) {
        if let Some(compliment) = set.get(&(sum_num - num)) {
            println!("compliment: {}, num: {}", compliment, num);
            answer = compliment * num;
            break;
        } else {
            set.insert(num);
        }
    }
    println!("Answer part 1: {}", answer);

    // let set: Rc<RefCell<HashSet<usize>>> = Rc::new(RefCell::new(HashSet::new()));
    // set.borrow_mut()
    //     .insert(contents.lines().nth(0).unwrap().parse().unwrap());
    // for num in contents.lines().map(|str| str.parse::<usize>().unwrap()) {
    //     for num2 in Rc::clone(&set).borrow().iter() {
    //         println!("{}", num2);
    //         if let Some(compliment) = set.borrow().get(&(sum_num - num - num2)) {
    //             println!("compliment: {}, num: {}, num2: {}", compliment, num, num2);
    //             answer = compliment * num * num2;
    //             break;
    //         } else {
    //             set.borrow_mut().insert(num);
    //         }
    //     }
    // }
    let mut items: Vec<usize> = contents
        .lines()
        .map(|i| i.parse().unwrap())
        .collect();
    items.sort_unstable();

    for a in &items {
        for b in &items {
            for c in &items {
                if a + b + c == 2020 {
                    answer = a * b * c;
                    break;
                }
            }
        }
    }
    println!("Answer part 2: {}", answer);
}
