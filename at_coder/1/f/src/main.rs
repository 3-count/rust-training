use std::io;
use regex::Regex;

fn main() {
    let mut line: String = String::new();
    io::stdin().read_line(&mut line).ok();
    let re = Regex::new(r"([A-Z][a-z]*[A-Z])").unwrap();
    let mut matches = Vec::new();
    let replaced = line.trim().replace("\r", "");
    for m in re.find_iter(replaced.as_str()) {
        matches.push(m.as_str());
    }

    matches.sort_by(|a, b| a.to_lowercase().cmp(&b.to_lowercase()));
    println!("{}",  matches.iter().map(|s| s.to_string()).collect::<Vec<_>>().join(""));
}
