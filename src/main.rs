use std::collections::HashMap;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    let n: i32 = match args[1].parse() {
        Ok(yay) => yay,
        Err(_e) => 100
    };

    println!();

    let mut reqs = HashMap::new();
    reqs.insert(3, "Fizz");
    reqs.insert(5, "Buzz");

    do_the_for(n, &reqs);
}

fn do_the_for(max: i32, map: &HashMap<i32, &str>) {
    for n in 1..max {
        let mut out = String::new();
        let mut works = False;

        for (key, value) in map {
            if n % key == 0 {
                out += &value.to_string()[..];
                works = True;
            }
        }

        if !works {
            println!("{}", n);
        } else {
            println!("{}", out);
        }
    }
}
