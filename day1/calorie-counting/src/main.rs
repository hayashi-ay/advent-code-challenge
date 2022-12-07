use std::io::{self, Read};

fn main() {
    let mut input = String::new();

    io::stdin()
        .read_to_string(&mut input)
        .expect("Failed to read line");

    let lines = input.split('\n');

    let mut calories: i64 = 0;
    let mut max: [i64; 3] = [0; 3];

    for line in lines {
        if line.is_empty() {
            max[0] = if max[0] > calories { max[0] } else { calories };
            max.sort();

            calories = 0;
            continue;
        }
        calories += line.parse::<i64>().unwrap();
    }
    println!("{}", max[0] + max[1] + max[2]);
}
