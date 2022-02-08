fn main() {
    // let mut x = [0];
    // let mut s = String::from("Hello");
    // s 在被赋值后不再有效
    // let mut s1 = s;
    // println!("{}", s);

    // let s2 = s.clone();
    // println!("{} {}", s, s2);

    //     let s = String::from("hello"); // s 进入作用域

    //     takes_ownership(s); // s 的值移动到函数里 ...
    //                         // ... 所以到这里不再有效
    //     println!("{}", s);

    //     let x = 5; // x 进入作用域

    //     makes_copy(x); // x 应该移动函数里，
    // }
    // let mut s = String::from("Hello, World");

    // let len = calculate_length(&mut s);
    // println!("{} {}", s, len);

    // let x = s;
    // println!("{}", x);
    let x = String::from("23");

    makes_copy(x);

    let y = x;

}

// fn takes_ownership(some_string: String) {
//     // some_string 进入作用域
//     println!("{}", some_string);
// } // 这里，some_string 移出作用域并调用 `drop` 方法。占用的内存被释放

fn makes_copy(some_integer: String) {
    // some_integer 进入作用域
    println!("{}", some_integer);
} // 这里，some_integer 移出作用域。不会有特殊操作

// fn calculate_length(s: &mut String) -> usize {
//     s.push_str("12");
//     s.len()
// }
