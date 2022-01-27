fn main() {
    // 默认声明的变量为不可变变量
    // let x = 6;
    // println!("{}", x);
    // x = 19; // cannot assign twic to immutable variable
    // let mut x = 1;
    // x = 23;

    // // 声明一个常量
    // const PI: f32 = 3.1415926;

    // // 数据类型
    // // 整数，浮点数，布尔型，字符
    // let x = 1.2222;
    // let y = 1.1111;
    // println!("{}", x + y);

    // // 元组
    // let temp = (23, 43, 23.1);
    // println!("{:?}", temp);

    // // 数组
    // let a: [i32; 2] = [1, 2];
    // println!("{:?}", a);
    // // 数据初始化
    // let b = [21; 12];
    // println!("{:?}", b);

    // 方法
    another_function(12);
    let y = {
        let x = 3;
        x + 1
    };

    println!("The value of y is: {}", y);
    println!("{}", returnFive());
}

fn another_function(x: i32) {
    println!("{}", x);
}

fn returnFive() -> i32 {
    5
}
