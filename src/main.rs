use std::io::{self, Read};

fn main() {
    let mut elements = String::new();

    io::stdin()
        .read_to_string(&mut elements)
        .expect("Failed to read line");

    let mut parts = elements.split_whitespace();

    let a: i64 = parts.next().expect("Нет числа").parse().expect("Не число");

    let b: i64 = parts.next().expect("Нет числа").parse().expect("Не число");

    println!("{}", a + b);
}
