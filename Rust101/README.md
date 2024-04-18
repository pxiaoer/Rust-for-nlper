
# Rust 101


## 安装

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

## 基础语法


### 从helloworld开始


创建：

```bash
cargo  new --bin helloworld 
```

生成helloworld项目结构：

```bash
helloworld
    ├── Cargo.toml
    └── src
        └── main.rs
```


cargo.toml内容：
```toml
[package]
name = "helloworld"
version = "0.1.0"
edition = "2021"

# See more keys and their definitions at https://doc.rust-lang.org/cargo/reference/manifest.html

[dependencies]
```

- name 名字
- version 版本
- edition 版次  目前最新为2021



再看src/main.rs内容：

```rust
fn main() {
    println!("Hello, world!");
}
```

很简单的，打印helloworld输出。

但是要注意，fn main是程序的入口，类似于C语言的main函数，但是不同的是，Rust的main函数不需要返回值，也不需要写return，最后一行的分号也可以省略。


编译：

```cargo build```

运行：
```cargo run```


打印的结果：

```bash
(base) ➜  helloworld git:(main) ✗ cargo run
    Finished dev [unoptimized + debuginfo] target(s) in 0.04s
     Running `target/debug/helloworld`
Hello, world!
```



### 基础类型 

rust使用let赋值

- 数字类型： 整数，浮点数
- 布尔类型： true，false
- 字符类型： char，Unicode  
- 字符串类型： String，&str
- 数组
- 动态数组
- 哈希表

### 复合类型

- 元组
- 结构体
- 枚举

### 控制语句
- 分支
- 循环

### 函数和模块
- 函数
- 闭包
- 模块


## 所有权

### 复制还是移动

默认做复制所有权的操作的有 7 种。
- 所有的整数类型，比如 u32；
- 布尔类型 bool；
- 浮点数类型，比如 f32、f64；
- 字符类型 char；
- 由以上类型组成的元组类型 tuple，如（i32, i32, char）；
- 由以上类型组成的数组类型 arrry，如 [9; 100]；
- 不可变引用类型 &

其他类型默认都是做移动所有权的操作



### 借用与引用