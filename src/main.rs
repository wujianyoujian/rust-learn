fn main() {
    let mut s = String::from("hello");
    let s1 = &s;
    print!("{}", s);
    change(&mut s);
    println!("{}", s);
    let slice = &s[0..2];
    println!("{}", slice);
}

// 参数为引用类型的字符串, 不会影响所有权
fn gives_owner_ship(a: &String) -> String {
    a.to_string()
}

fn change(val: &mut String) -> String {
    val.to_string()
}