use std::collections::HashMap;

fn main() {
    println!();

    let mut reqs = HashMap::new();
    reqs.insert(3, "Fizz");
    reqs.insert(5, "Buzz");

    doTheFor(100, &reqs);
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
