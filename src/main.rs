// 显示引入io到当前作用域中
use std::io;
use rand::Rng;

// main是程序的入口
fn main() {
    println!("Guess the number");

    println!("Please input your guess");

    let secret_number = rand::thread_rng().gen_range(1..101);

    println!("The secret number is: {}", secret_number);
    
    // mut 可变修饰，变量默认
    let mut guess = String::new();

    io::stdin()
        .read_line(&mut guess)
        .expect("Faild to read line");

    println!("You guessed: {}", guess);
}
