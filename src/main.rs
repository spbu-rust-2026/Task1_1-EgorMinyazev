use std::io;

fn main() {
    let mut elements = String::new();
    
    io::stdin()
        .read_line(&mut elements)
        .expect("Failed to read line");
    
    let mut parts = elements.split_whitespace();

    let a: i32 = parts
        .next()
        .expect("Нет числа")
        .parse()
        .expect("Не число");

    let b: i32 = parts
        .next()
        .expect("Нет числа")
        .parse()
        .expect("Не число");
    p
    rintln!("{}", a+b);
}
