use std::io;

fn main() {
    let mut line: String = String::new();
    io::stdin().read_line(&mut line).ok();
    let number = line.trim().replace("\r", "").parse::<i32>();
    if number.is_err() {
        println!("error");
        return;
    }

    println!("{}", number.unwrap() * 2);
}
