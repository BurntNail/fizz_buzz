use std::collections::HashMap;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    let n: i32 = args[1].parse().unwrap_or(100);

    let reqs = HashMap::from([
        (3, "Fizz".into()),
        (5, "Buzz".into())
    ]);

    println!("{}", do_the_for(n, reqs));
}

fn do_the_for<T: IntoIterator<Item = (i32, String)> + Clone> (max: i32, map: T) -> String {
    (1..=max).into_iter().map(|n| {
        let mut out = String::new();
        
        for (key, value) in map.clone() {
            if n % key == 0 {
                out += &value;
                worked = true;
            }
        }

        if out.is_empty() {
            out += &n.to_string();
        }
        out + "\n"
    }).collect()
    }
