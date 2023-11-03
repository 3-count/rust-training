use std::io;

fn main() {
    let mut line: String = String::new();
    io::stdin().read_line(&mut line).ok();
    let mut numbers: Vec<i32> = line.trim().replace("\r", "").split_whitespace().map(|p| p.parse::<i32>().unwrap()).collect();
    numbers.sort();
    numbers.reverse();
    println!("{}", numbers[2]);
}