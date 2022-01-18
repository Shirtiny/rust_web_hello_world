/*
 * @Author: Shirtiny
 * @Date: 2022-01-05 10:06:31
 * @LastEditTime: 2022-01-18 20:24:17
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
    let sum = |a: i32, b: i32| a + b;
    let sum2 = |a: i32, b: i32| -> i32 { return a + b };
    println!("简写 {}, 全写{}", sum(3, 4), sum2(3, 4));
}

#[derive(Debug)]
#[allow(dead_code)]
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

#[allow(dead_code)]
#[derive(Debug)]
struct Rect {
    height: i32,
    width: i32,
}

#[allow(dead_code)]
impl Rect {
    // 构造函数 可以写外面
    pub fn new(height: i32, width: i32) -> Self {
        Rect { height, width }
    }
    // 类似静态方法 跟this没关系
    fn area_func(w: i32, h: i32) -> i32 {
        w * h
    }
    // 不可变的this
    fn area(&self) -> i32 {
        self.height * self.width
    }
    // 可变的this
    fn height_mut(&mut self, n: i32) -> i32 {
        self.height += n;
        return self.height;
    }
    // 用过后会销毁
    fn use_then_drop(self) {
        println!("drop");
    }
}

fn _run_struct() {
    let rect = Rect {
        height: 200,
        width: 300,
    };
    println!("{:?}, {}, {}", rect, rect.height, rect.width);

    let rect2 = Rect::new(10, 20);
    println!(
        "{:?},  {}, {}, {}",
        rect2,
        rect2.area(),
        Rect::area_func(10, 20),
        Rect::area(&rect2)
    );

    let mut rect3 = Rect::new(20, 20);
    rect3.height_mut(3);
    println!("{:?}, {}", rect3, rect3.height);
    // 执行完这个函数 rect3就被销毁了
    rect3.use_then_drop();
}

fn main() {
    // _read_input();
    // _prints();
    // _run_fn();
    // _run_enum();
    // _run_struct();
}
