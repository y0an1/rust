# 安装
- 官网：https://www.rust-lang.org
- 更新：```rustup update```
- 卸载：```rustup uninstall```
- rust 程序：
    - 程序后缀名：```rs```
    - 文件名规范：```hello_world.rs```

# 项目管理
- cargo：```cargo --version```
- 创建项目：```cargo new project_name```
- 构建项目：```cargo build```
    - 直接运行构建项目命令是 debug 版本，如果要 release 版本则需要添加```--release```
- 运行项目：```cargo run```
- 检查项目代码：```cargo check```

## Cargo.tomal
- cargo 项目的配置文件
- **在 Rust 中，一个包（项目）通常被称作 crate。如一个第三方的库，则称一个 crate**
```
# 用来配置包（项目）
[package]
name = "hello_cargo"
version = "0.1.0"
edition = "2021"

# See more keys and their definitions at https://doc.rust-lang.org/cargo/reference/manifest.html

# 包（项目）所使用的依赖
[dependencies]
```

# 变量与可变性
1. rust 中，变量是默认不可变的，如果一个变量的值后续要进行变化，则需要添加 “mut” 关键字