
# Rust 101


## 安装

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

## 基础语法


### 从helloworld开始


创建：
'''bash
cargo  new --bin helloworld 
'''

生成helloworld项目


项目结构：
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


src/main.rs内容：

```rust
fn main() {
    println!("Hello, world!");
}
```

很简单的helloworld输出，但是要注意，fn main是程序的入口，类似于C语言的main函数，但是不同的是，Rust的main函数不需要返回值，也不需要写return，最后一行的分号也可以省略。


编译：

cargo build

运行：
cargo run 


```bash
(base) ➜  helloworld git:(main) ✗ cargo run
    Finished dev [unoptimized + debuginfo] target(s) in 0.04s
     Running `target/debug/helloworld`
Hello, world!
```