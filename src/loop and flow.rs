fn main() {
    // let number = 3;

    // if number < 3 {
    //   println!("condition was true");
    // } else {
    //   println!("condition was false");
    // }
    // let number = if true { 5 } else { 4 };

    // 循环
    // loop, for, while
    // loop {
    //   println!("agin");
    // }

    // 循环标签
    // let mut count = 1;
    // 'counting_up: loop {
    //   if count > 10 {
    //     break 'counting_up;
    //   }
    //   count += 1;
    // }
    // println!("{}", count)

    // 循环返回值
    // let mut counter = 0;

    // let result = loop {
    //     counter += 1;

    //     if counter == 10 {
    //         break counter * 2;
    //     }
    // };

    // println!("The result is {}", result);
    let a = [1, 2, 4, 5];
    for element in a {
        print!("{} ", element);
    }
    println!("{}", "");

    for number in (1..4).rev() {
        print!("{} ", number);
    }

    println!("{}", "");

    let arr = [1;20];
    println!("{:?}", arr);
}
