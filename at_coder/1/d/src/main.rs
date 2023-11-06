use std::io;
use std::collections::HashSet;

fn main() {
    let mut line: String = String::new();
    io::stdin().read_line(&mut line).ok();
    let count: i32 = line.trim().replace("\r", "").parse::<i32>().unwrap();
    let mut number_set: HashSet<i32> = HashSet::new();
    let mut uninput_set: HashSet<i32> = HashSet::new();
    let mut duplicate: i32 = 0;
    for n in 1..count + 1 {
        line.clear();
        io::stdin().read_line(&mut line).ok();
        let number: i32 = line.trim().replace("\r", "").parse::<i32>().unwrap();
        if number_set.contains(&number) {
            duplicate = number;
        } else {
            number_set.insert(number);
        }

        if !number_set.contains(&n) {
            uninput_set.insert(n);
        }

        uninput_set.remove(&number);
    }

    if uninput_set.is_empty() {
        println!("Correct");
        return;
    }

    println!("{} {}", duplicate, uninput_set.iter().last().unwrap());
}