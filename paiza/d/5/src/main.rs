use std::io;

fn main() {
    let mut line: String = String::new();
    io::stdin().read_line(&mut line).ok();
    let parameters: Vec<i32> = line.trim().split_whitespace().map(|p| p.parse().unwrap()).collect();
    let mut value: i32 = parameters[0];
    print!("{}", value);
    for _n in 1..10 {
        value += parameters[1];
        print!(" {}", value);
    }

    println!("");
}