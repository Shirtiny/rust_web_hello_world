/*
 * @Author: Shirtiny
 * @Date: 2022-01-05 10:06:31
 * @LastEditTime: 2022-01-06 11:05:59
 * @Description:
 */

fn main() {
    println!("{} days {}", 31, 31i64);
    println!("{0}, this is {1}. {1}, this is {0}", "A", "B");
    println!("{a}, this is {b}. {b}, this is {a}", a = "Alice", b = "Bob");
    // Special formatting can be specified after a `:`.
    println!(
        "{} of {:b} people know binary, the other half doesn't",
        1, 2
    );
}
