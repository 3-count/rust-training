use std::io;
use std::collections::HashMap;

fn main() {
    let mut line: String = String::new();
    io::stdin().read_line(&mut line).ok();
    let parameters: Vec<&str> = line.trim().split_whitespace().collect();
    let value: i32 = parameters[0].parse().unwrap();
    let unit_name = parameters[1];
    let to_millimeter_map = HashMap::from([
        ("km", 1000 * 1000),
        ("m", 1000),
        ("cm", 10)
    ]);
    println!("{}", to_millimeter_map[unit_name] * value);
}