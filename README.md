# Rust

## 命令
### 创建项目
> `cargo new <project_name>`

### 构建项目
* 根据配置文件，会下载依赖
> `cargo build`

### 运行项目
* 会根据项目下`src/main.rs`文件的`main`入口函数进行执行
> `cargo run`

## 变量
* 默认不可变

## 类型
* 数字类型
> `i32` 有符号的32位  
> `u32`  无符号的32为，表示一直为正整数
* 浮点数类型
> `f32`
* 布尔型
> `false`, `true`
* 字符型
> `''`
* 元组
* 数组
> [ ] 只能是同一个类型的值

## 方法
* 定义一个方法
```rust
// 参数类型为整数的方法
fn another_function(x: i21) {

}
// 返回类型为32位整数类型
fn another_function1() -> i32 {
  23
}
```
## 流程
