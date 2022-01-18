/*
 * @Author: Shirtiny
 * @Date: 2022-01-05 10:06:31
 * @LastEditTime: 2022-01-18 14:18:55
 * @Description:
 */
use std::io;

fn _read_input() -> String {
    let mut input = String::new();
    println!("input something");
    match io::stdin().read_line(&mut input) {
        Ok(_) => {
            println!("hello, {}", input)
        }
        _ => {}
    }
    return input;
}

fn _prints() {
    println!("{} days {}", 31, 31i64);
    println!("{0}, this is {1}. {1}, this is {0}", "A", "B");
    println!("{a}, this is {b}. {b}, this is {a}", a = "Alice", b = "Bob");
    // Special formatting can be specified after a `:`.
    println!(
        "{} of {:b} people know binary, the other half doesn't",
        1, 2
    );
}

fn _add(a: i32, b: i32) -> i32 {
    a + b
}

fn _run_fn() {
    println!("{}", _add(1, 2));
    let sum = |a: i32, b: i32| -> i32 { a + b };
    println!("{}", sum(3, 4));
}

#[derive(Debug)]
enum Position {
    Left,
    Right,
}

fn _run_enum() {
    println!("{:?}", Position::Right);
    let pos = Position::Left;
    match pos {
        Position::Left => {
            println!("{:?}", Position::Left);
        }
        Position::Right => {}
    }
}

fn main() {
    // _read_input();
    // _prints()
    // _run_fn()
    _run_enum()
}
