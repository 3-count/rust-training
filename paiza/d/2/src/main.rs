use std::io;

fn main() {
    let mut line: String = String::new();
    io::stdin().read_line(&mut line).ok();
    let numbers: Vec<i32> = line.split_whitespace().map(|x| x.parse::<i32>().unwrap()).collect();
    if numbers[0] == numbers[1] {
        println!("eq");
        return;
    }

    println!("{}", numbers.iter().max().unwrap());
}
