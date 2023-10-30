use std::io;

fn main() {
    let mut input: String = String::new();
    io::stdin().read_line(&mut input).ok();
    let count: i32 = input.trim().parse().unwrap();
    let mut words: Vec<String> = vec![];
    for _n in 0..count {
        input.clear();
        io::stdin().read_line(&mut input).ok();
        words.push(input.trim().replace("\r", ""));
    }

    println!("Hello {}.", words.join(","));
}