use std::collections::HashMap;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    let n: i32 = args[1].parse().expect("Not a valid 32 bit signed integer.");

    println!();

    let mut reqs = HashMap::new();
    reqs.insert(3, "Fizz");
    reqs.insert(5, "Buzz");

    doTheFor(n, &reqs);
}

fn doTheFor(max: i32, map: &HashMap<i32, &str>) {
    for n in 1..max {
        let mut out = String::new();

        for (key, value) in map {
            if n % key == 0 {
                out += &value.to_string()[..];
            }
        }

        if out == String::new() {
            println!("{}", n);
        } else {
            println!("{}", out);
        }
    }
}
