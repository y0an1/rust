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

# 通用编程概念
## 变量与可变性
- Rust 中，变量是默认不可变的，如果一个变量的值后续要进行变化，则需要添加 “mut” 关键字

### 变量与常量
- 常量（constant）：常量在绑定值后也是不可变的，但与不可变的变量还是有区别：
    - 不可以使用 **mut**
    - 声明常量使用 **const** 关键字，它的类型必须标注
    - 常量可以在任何作用域进行声明
    - 常量只可以绑定到常量表达式，无法绑定到函数的调用结果或只能在运行时才能计算出的值
- 在程序运行期间，常量在其声明的作用域内一直有效
- 命名规范： Rust 里常量使用全大写字母，每个单词之间用下划线分开，例如：
    - MAX_POINTS: ```const MAX_POINTS: u32 = 100_000;```

## 数据类型
### 标量类型

### 符合类型


## 函数

## 注释

## 控制流