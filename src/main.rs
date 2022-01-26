// 显示引入io到当前作用域中
use std::io;

// main是程序的入口
fn main() {
    println!("Guess the number");

    println!("Please input your guess");

    // mut 可变修饰，变量默认
    let mut guess = String::new();

    io::stdin()
        .read_line(&mut guess)
        .expect("Faild to read line");

    println!("You guessed: {}", guess);
}
