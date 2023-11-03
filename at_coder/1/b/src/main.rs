use std::io;

fn main() {
    let mut line: String = String::new();
    io::stdin().read_line(&mut line).ok();
    let number = line.trim().replace("\r", "").parse::<i32>().unwrap();
    let mut previous: i32 = 0;
    let mut results: Vec<String> = Vec::new();
    for n in 0..number {
        line.clear();
        io::stdin().read_line(&mut line).ok();
        let current = line.trim().replace("\r", "").parse::<i32>().unwrap();
        let compare = current - previous;
        previous = current;
        if n == 0 {
            continue;
        }

        if 0 < compare {
            results.push(format!("up {}", compare));
            continue;
        }

        if compare < 0 {
            results.push(format!("down {}", -compare));
            continue;
        }

        results.push(String::from("stay"));
    }

    for result in results {
        println!("{}", result);
    }
}