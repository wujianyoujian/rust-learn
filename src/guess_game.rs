// 显示引入io到当前作用域中
use rand::Rng;
use std::cmp::Ordering;
use std::io;

// main是程序的入口
fn main() {
    println!("Guess the number");

    let secret_number = rand::thread_rng().gen_range(1..101);

    loop {
        println!("Please input your guess");

        // mut 可变修饰，变量默认
        let mut guess = String::new();
        io::stdin()
            .read_line(&mut guess)
            .expect("Faild to read line");
        println!("You guessed: {}", guess);
        
        // 将字符类型的guess转换为数字类型
        // let value = guess.trim().parse::<u32>().expect("please type a number");
        // parse 返回Result 类型，Result是一个具有Ok,Err 的枚举，可以使用匹配表达式
        let value = match guess.trim().parse::<u32>() {
            Ok(num) => num,
            Err(_) => continue,
        };

        match value.cmp(&secret_number) {
            Ordering::Less => println!("too small"),
            Ordering::Greater => println!("Too big"),
            Ordering::Equal => {
                println!("You win");
                break;
            }
        }
    }
}
