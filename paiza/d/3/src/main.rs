use std::io;

fn main() {
    let mut line: String = String::new();
    io::stdin().read_line(&mut line).ok();
    let number: i32 = line.trim().parse().unwrap();
    for n in 1..10 {
        let mut separator : &str = " ";
        if n == 9 {
            separator = "\r\n";
        }

        print!("{}{}", number * n, separator);
    }
}
