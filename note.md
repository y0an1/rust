# 安装

- 官网：<https://www.rust-lang.org>
- 更新：```rustup update```
- 卸载：```rustup uninstall```
- rust 程序：
  - 程序后缀名：```rs```
  - 文件名规范：```hello_world.rs```
- link.exe no find 错误，可以通过 `rustup toolchain install stable-x86_64-pc-windows-gnu` 和 `rustup default stable-x86_64-pc-windows-gnu` 解决

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

# No.3 通用编程概念

## 3.1 变量与可变性

- Rust 中，变量是默认不可变的，如果一个变量的值后续要进行变化，则需要添加 “mut” 关键字

### 变量与常量

---

- 常量（constant）：常量在绑定值后也是不可变的，但与不可变的变量还是有区别：
  - 不可以使用 **mut**
  - 声明常量使用 **const** 关键字，它的类型必须标注
  - 常量可以在任何作用域进行声明
  - 常量只可以绑定到常量表达式，无法绑定到函数的调用结果或只能在运行时才能计算出的值
- 在程序运行期间，常量在其声明的作用域内一直有效
- 命名规范： Rust 里常量使用全大写字母，每个单词之间用下划线分开，例如：
  - MAX_POINTS: ```const MAX_POINTS: u32 = 100_000;```

### Shadowing(隐藏)

---

- Rust 中，可以使用相同的名字声明新的变量，新的变量就会 shadow（隐藏）之前声明的同名变量，当后面的代码使用该变量时，就会使用新的，而不是旧的
- Shadow 和把变量标记为 **mut** 是不一样的
  - 如果不使用 **let** 关键字，那么重新给非 **mut** 的变量赋值会导致编译时错误
  - 而使用 **let** 声明的同名新变量，也是不可变的
  - 使用 **let**声明的同名新变量，它的类型可以与之前不同

## 3.2 数据类型

- 一般情况下，编译器都可以自动的推导出变量的数据类型，但当有多个数据类型符合一个变量时，则必须手动的设置

### 标量类型

---

- 一个标量类型代表一个单个值
- Rust 有四个主要的标量类型：
  - 整数类型
    - 无符号整数类型，以**u**开头
    - 有符号整数类型，以**i**开头
    - 一共有 5 种类型。8,16,32,64,128, **arch**.
    - **arch**：分别是“isize”和“usize”，**size** 的具体位数是由程序运行时的计算机架构决定的
    - 整数字面值：除了 byte 类型外，所有的数值字面值都允许使用类型后缀，例如：**57u8**：57是数值，u8是类型
  - 浮点类型
    - **f32**: 单精度
    - **f64**：双精度
  - 布尔类型
    - **true** 和 **false**
  - 字符类型
    - 占用 4 字节大小
    - 默认是 Unicode 标量值，而不是 ASCII 标量值

### 复合类型

---

- 将多个值存放到一个类型里
- Rust 提供两种基础的复合类型：元组（Tuple）、数组
  - 元组：
    - 可以将多个类型的值放到一个类型里
    - 长度固定
    - 创建 Tuple: ```let tup: (i32, f64, u8) = (500, 6.4, 1);```
    - 获取 Tuple 的元素值：```let (x, y, z) = tup; // 使用模式匹配来解构 Tuple，从而获取到元素的值```
    - 访问 Tuple 的元素：```println!("{}, {}, {}", tup.0, tup.1, tup.2); // 使用点标记法来访问元素的值```
  - 数组：
    - 创建数组：```let ary = [1,2,3,4,5,6];```
    - 数组的类型：```let ary2:[i32; 3] = [1,2,3]; // 显式的声明数组类型[type; len]```
    - 一般用于初始化数组： ```let ary3 =[0;3]; // 相当于 int ary[3] = {0};```
    - 访问数组的元素跟其他语言一样，但数组越界的处理上，Rust 是不允许的，具体表现为：**编译会通过**，**运行时会报错**

## 3.3 函数

- 函数使用 **fn** 关键字
- 函数必须声明每个参数的类型
- 函数的返回值类型必须在函数定义时，在 ‘->’ 后面声明。**函数的返回值不能被命名**

## 3.4 控制流

### if

---

- 在 rust 中，if 的 condition 中不需要添加‘()’
- 在 rust 中，if 的 condition 必须是布尔值
- 在 rust 中，if-else 的返回值类型必须是一样

### 循环

---

- loop：死循环，直到 break 条件生效
  - 如果 break 后面有跟表达式，则该表达式的值就是这个 loop 的返回值
- while
  - 跟其他语言的 while 循环基本一样
- for
  - 一般是用于遍历集合

#### Range

---

- 标准库提供的一个库，指定一个开始数字和结束数字， Range 可以生成它们之间的数字（[start_num .. end_num)）
- rev 方法可以反转 Range

# No.4 所有权

- 为了解决内存安全的问题，不再借助 GC 来进行管理（Java），或由程序员显示分配和释放内存（C、C++）
- 内存将有一个所有权系统来进行管理，其中包含一组编译器在编译时检查的规则，而因为检查是在编译期进行的，所以不会在运行期产生任何的开销
- 所有权解决的问题：
  - 跟踪代码的哪些部分正在使用 heap 的哪些数据
  - 最小化 heap 上的重复数据量
  - 清理 heap 上未使用的数据以避免空间不足
  - 所以所有权存在的原因是为了管理 heap 数据

## 4.1.1 所有权的规则

1. 每个值都有一个变量，该变量是该值的所有者
2. 每个值同时只能一个所有者
3. 当所有者超出作用域（scope）时，该值将被删除

### Drop 函数

---

- 当一个变量指向堆内存时，该变量离开作用域，rust 会自动调用 drop 函数

### 变量和数据交互的方式：移动（Move）

---

- 在 Rust 中，如果一个变量是指向栈内存，那么当其作为等号右值时，其值会被复制（拷贝）；当变量是指向堆内存，那么当作为右值时，其值会被移动到左值，该变量会变成无效。

### 变量和数据交互的方式：克隆（Clone）

---

- Rust 设计规则：Rust 不会自动创建数据的深拷贝
- clone 方法是对数据进行深拷贝

### 栈内存上的数据：复制（Copy）

---

- 当一个类型实现了 **Copy** 这个 trait， 那么旧的变量在赋值后仍然可用
- Rust 中规定：当一个类型或该类型的一部分实现了 drop 这个函数，那么 copy 则不能被实现
- 内置 copy 函数的类型：
  - 所有整型
  - bool
  - char
  - 所有浮点类型
  - Tuple 中，如果所有字段都是 Copy 的，那么其就可以 copy，否则不行。
    - (i32, i32) - 可以
    - (i32, String) - 不可以

## 4.1.2 所有权与函数

- 函数传参会与赋值一样均发生 **移动** 或 **复制** 的情况

### 返回值与作用域

---

- 返回值会发生所有权的转移

### 变量的所有权

---

- 把一个值赋值给其他变量时， 会发生移动
- 当一个包含 heap 数据的变量离开作用域时，它的值就会被 drop 函数 清理，除非数据的所有权移动到另一个变量上

## 4.2 引用与借用

### 引用

---

- Rust 为了避免变量的所有权的改变，而需要多次倒腾变量，提出了“引用”的特性，允许引用某些值而不取得所有权
- 使用引用， “&” 关键字

### 借用

---

- 当把引用作为函数参数时， 这个行为叫做借用
- 不能修改借用的东西， 因为引用默认是不可变的

### 可变引用

---

- **&mut** 可以将一个引用标示为可变引用
- **限制：在同一个作用域内，对某块数据，有且仅有一个可变的引用**，这样做的好处是为了防止数据竞争
  - 发生数据竞争的三种行为：
    1. 两个说多个指针同时访问同一个数据
    2. 至少有一个指针用于写入数据
    3. 没有使用任何机制来同步对数据的访问
- 可以通过创建新的作用域，来允许非同时的创建多个可变引用
- **限制：不可以同时拥有一个可变引用和不可变引用**

### 悬空引用（Dangling References）

---

- **一个指针引用了内存中的某个地址，而这块内存可能已经释放并分配给其他人使用**
- 在 Rust 中，编译器则确保了不会发生这种情况

### 引用的规则

---

- 在任何给定的时刻，只能满足下列条件之一：
  - 一个可变的引用
  - 任意数量不可变的引用
- **引用必须一直有效**

## 4.3 切片（slice）

- Rust 中另一种不持有所有权的数据类型

### 字符串切片

---

- 指向字符串中的一部分引用， 当该字符串无效时，其切片也随之无效 
- 形式： ```&str[start_index..end_index]```
  - **start_index**: 切片起始位置的索引
  - **end_index**: 切片终止位置的下一个索引值
- 语法糖
  - 如果 start_index 从 0 开始，则可以不写
  - 如果 end_index 是字符串的长度，也可以不写
- **注意**
  - 字符串切片的范围索引必须发生在有效的 UTF-8 字符边界内
  - 当尝试从一个多字节的字符中创建字符串切片，程序会报错并退出
- 字符串切片的类型： **&str**
- 将字符串切片作为函数参数，可以同时接收 String 和 &str 类型的参数

# No.5 Struct

## 5.1 定义和实例化 struct

### 定义 struct

---

```rust
struct User {
  username:         String,
  email:            String,
  sign_in_count:    u64,
  active:           bool,   // 注意：最后一个字段也是以 “,” 结尾
}
```

### 实例化 struct

---

```
let user = User {
    email: String::from("someone@example.com"),
    username: String::from("someusername123"),
    active: true,
    sign_in_count: 1,
};
```

### struct 更新语法

---

```
let user1 = User {
    email: String::from("someone@example.com"),
    username: String::from("someusername123"),
    active: true,
    sign_in_count: 1,
};

let user2 = User {
  email: String::from("someone@example.com"),
  username: String::from("someusername123"),
  ..user1,
};
```

### Tuple struct

---

- 类似 tuple 的 struct，即：struct 整体有名，而字段没有名

```
struct Color(i32, i32, i32);
struct Point(i32, i32, i32);

let black = Color(0, 0, 0);
let origin = Point(0, 0, 0);
```

### Unit-Like Struct

---

- 没有任何字段的结构体
- 适用于需要在某个类型上实现某个 trait，但在里面又没有想要存储的数据

### struct 所有权

---

```rust
struct User {
  username: String,
  email: String,
  sign_in_count: u64,
  active: bool,
}
```

- User 这个 struct 的字段是使用了 String，而不是 &str，所以
  - 该 struct 实力拥有其所有的数据
  - 只要 struct 实例是有效的，那么里面的字段数据也是有效的
- 当 struct 的字段是引用时，涉及到生命周期（后面讲解）
  - 生命周期保证只要 struct 实例是有效的，那么引用字段就是有效的 
  - 如果 struct 里面的字段是引用，而没有使用生命周期，程序就会报错

### 打印结构体

---

- 如果想要使用 **println!("{}", struct)** 来打印一个 struct 实例，必须实现 **std::fmt::Display** 函数，否则实现 **std::fmt::Debug** 函数
- 如果以上两个函数都没有实现，但却要调试信息，则可以在声明结构体时，在其上面添加 **#[derive(Debug)]** 后，后续则可以用 **{:?}** 或者 **{:#?}** 来打印出当前的结构体实例的结构

## 5.2 struct 方法

- 与函数类似，不同之处：
  - 方法在 struct(或 enum、trait 对象)的上下文中定义
  - 第一个参数是 **self**

### 定义方法

---

- 在 **impl** 块里定义方法
- 方法的第一个参数可以是 **&self**，也可以是 **self**，或者是 **mut self** / **&mut self**，跟普通的参数一样

### 方法调用

---

- 在 Rust 中，并没有像 C/C++ 一样，有 “->” 和 “.” 的区别。Rust 只有 “.”，编译器会自动识别当前的方法调用，自动添加引用或解引用


### 关联函数

---

- 类似于 C++ 的静态方法
- 在 **impl**( implement ) 块里定义函数，但函数的第一个参数不再是 **self** 的，则是 **关联函数**
- 关联函数一般情况下，用于 struct 的构造器，例如： String::from() --- 就是一个关联函数

# No.6 枚举与模式匹配

## 6.1 定义枚举

- 使用关键字 **enum** 定义

```rust
enum IpAddrKind {
  V4,   // 枚举的变体
  V6,
}
```

- 将数据附加到枚举的变体中
  - 优点： 1. 不需要额外使用 struct，2. 每个变体可以拥有不同的类型以及关联的数据量

```rust
enum IpAddr {
  V4(u8, u8, u8, u8),
  V6(String),
}
```

### 定义枚举的方法

---

- 跟 struct 的一样，使用 **impl** 块进行定义

## 6.2 Option 枚举

- 定义于标准库中， 在 Prelude(预导入模块)中，描述了：某个值可能存在(某种类型)或不存在的情况，主要是为了解决 NULL 的情况
- **在 Rust 中，没有 Null 值存在**
- Option<T> 和 T 是不同的类型，不可以把 Option<T> 直接当成 T，要使用 Option<T> 中的 T，必须手动进行转化为 T
  - 调用 **expect()** 方法即可转化为 T
- 标准库中的定义：
  - 在使用 Option<T> 时，如果要编译器推导类型，则必须设置 T，否则编译器无法推导出变量的类型
 
```rust
enum Option<T> {
  Some(T),
  None,
}
```

## 6.3 控制刘运算符 - match

- 允许一个值与一系列模式进行匹配，并执行匹配的模式对应的代码
- 模式可以是字面值、变量名、通配符...
- 使用 match 匹配时，必须穷举所有的可能性，如果不想处理其中的某几种可能，则可以使用 “_” 来表示

### 绑定值的模式

---

- 匹配的分支可以绑定到被匹配对象的部分值，因此，可以从 enum 变体中提取值

## 6.4 if let

---

- 类似 match，但 if let 只关心一种匹配情况，而忽略其他情况


# No.7 Package, Crate, Module

## 7.1 Package、Create、定义 Module

- Package(包)：Cargo 的特性，构建，测试，共享 crate
- Crate(单元包)：一个模块树，它可产生一个 library 或可执行文件
- Module(模块)、use： 让你控制代码的组织、作用域、私有路径
- Path(路径)：为 struct、function 或 module 等项命名的方式

```
Package
  |
  |-> Crate
    |
    |-> Module
      |
      |-> Path
```

### Package 和 Crate

---

- **Crate** 的类型共有两种： **binary** 和 **library**
- **Crate Root**：是一个源代码文件，Rust 编译器是从该文件开始执行代码逻辑
- <font color=red>**Package**
  - 包含了一个 **Cargo.toml**，该文件描述了如何构建这些 **Crates**
  - 只能包含 0-1 个 library carte
  - 可以包含无数个 binary crate
  - 但必须至少包含一个 crate（library 或 binary）</font>

### Cargo 的惯例

---

- **src/main.rs**
  - 是 binary crate 的 crate root
  - crate 名和 package 名相同
- **src/lib.rs**
  - package 包含一个 library carte
  - 是 library carte 的 crate root
  - crate 名和 package 名相同
- Cargo 会把 carte root 文件移交给 rustc（Rust 编译器）来构建 library 或 binary
- 如果将文件放在 **src/bin** 下面，则每个文件都是单独的 binary crate

### Crate 的作用

---

- 将相关功能组合到一个作用域内，便于在项目间进行分享，还可以防止命名冲突

### 定义 Module 来控制作用域和私有性

---

- **Module**
  - 在一个 crate 内，将代码进行分组，增加可读性，易于复用
  - 控制条目（item）的私有性（public、private）
- 建立 module
  - 使用 **mod** 关键字
  - module 可以嵌套定义
  - 可以包含其它项（struct、enum、常量、trait、函数等）的定义

### Module

- **src/main.rs** 和 **src/lib.rs** 叫做 **carte roots**，它们（任意一个）的内容形成了名为 crate 的模块，位于整个模块树的根部

## 7.2 路径（Path）

- 跟其他语言中的命名空间有点类似
- 路径有两种形式
  - 绝对路径：从 crate root 开始，使用 crate 名或字面值 `crate`
  - 相对路径：从当前模块开始，使用 self，super或当前莫烤烟的标识符
- 如果路径中有多个标识符，标识符之间则用 **::** 分割

### 私有边界（privacy boundary）

---

- 模块不仅可以组织代码，还可以定义私有边界，如果把 **函数** 或 **struct** 等设为私有，可以将它放到某个模块中
- Rust 中所有的条目（函数，方法，struct，enum，module，常量）默认是 **私有的**
- 父级模块无法访问子模块中的私有条目
- 子模块里可以使用所有祖先模块中的条目

### pub 关键字

---

- pub struct
  - struct 是公共的
  - 结构体字段默认是 **私有的**
- pub enum
  - enum 是公共的
  - 枚举的变体也是 **公共的**

### use 关键字

---

- 可以使用 `use` 关键字将路径导入到作用域内
- 使用 `use` 关键字的约定俗成
  - 函数：将函数的父级模块引入到作用域（指定到父级）
  - struct、enum，其它：指定完整路径（指定到本身），但有同名条目时，需要指定到父级
- 可以使用 `pub use` 来向外暴露私有的模块，而外部再使用时，不用再重复导入

### as 关键字

---

- `as` 关键字可以为引入的路径指定本地的别名

### 使用外部包

---

1. Cargo.toml 中的 [dependencies] 中添加依赖的包名和版本，格式：`pkg_name = "version"`
2. `use` 将特定条目引入作用域

- 切换 crate 的下载地址
  - 到 **.cargo** 目录下，创建 **config** 文件，将下面的内容写入到文件中
  ```
  [source.cartes-io]
  registry = "https://github.com/rust-lang/crates.io-index"
  
  replace-with = 'tuna'
  [source.tuna]
  registry = "https://mirrors.tuna.tsinghua.edu.cn/git/crates.io-index.git"
  
  [net]
  git-fetch-with-cli = true
  ```
  
### 使用嵌套路径清理大量的 use 语句

---

- 格式： 
  - **路径相同的部分::{差异部分1， 差异部分2， 差异部分3，...}；**
  - **路径相同的部分::{self， 差异部分2， 差异部分3，...}；** 

### 将模块内容写入到其他文件

---

- 当`mod module_name` 后跟随的不再是 "{}" ，而是 ";"，Rust 则会在同级目录下寻找对应模块名的文件


# No.8 常用的集合

- 集合：存放在堆内存上的数据结构，分别：Vector、String、HashMap

## 8.1 Vector

- Vec<T>，可以存放多个值，只能存放相同类型的数据，在内存中连续存放，类似与 array
- 创建 vector: **Vec::new()**
  - 使用初始值创建 Vec<T>: **vec!**
- 获取 vector 的值： 1. 索引，2. get()
  - 需要注意的是： 当使用索引获取时，如果越界了，程序就会崩溃。而使用 get() 函数则不会，会返回 None 值
- 对 vector 的所有权和借用规则都与默认的一样，例子
- 利用 enum 的变体可以存放多种数据类型的特性， 创建一个枚举类型的 vector 来存放多种数据类型，例子

## 8.2 String

### 创建 String 的方式：

---

- 使用 String::new 函数
- 使用 to_string() 方法
- 使用 String::form() 函数

### 更新 String：

---

- 使用 **push_str()** 方法，将一个 **字符串切片** 附加到 String
- 使用 **push()** 方法，将 **单个字符** 附加到 String

### 拼接 String：

---

- 使用 **+** 运算符拼接 String，需要注意的是： + 号左边的是 **字符串类型**，右边需要的是一个 **字符串切片**
  - 首先：加法运算在 Rust 中是使用了泛型，所以此处实际上是调用了类似： `fn add(self, s: &str) -> String` ，所以使用了 + 进行字符串拼接后，首个字符串的所有权会被转移
  - 第二个参数明明是字符串切片，但传递字符串引用却可以调用，是因为编译器在传参时，进行了解引用强制转换(deref coercion)
  - 字符串拼接运算符 + 左边只能是一个 String 类型，不能是 & 或 字符串切片类型，否则编译器会提示： **note: string concatenation requires an owned `String` on the left**
- 使用 **format!** 宏拼接 String
  - 使用 `format!()` 不会获取字符串的所有权，后续如果要使用其参数，是可以正常使用

### String 按索引方式访问

---

- 在 Rust 中，字符串是 utf-8 的形式保存
- Rust 的字符串不支持索引语法访问，因为没有实现 `Index<{integer}>` 这个 trait
 
### 内部封装 

---

- String 实际上是对 Vec<u8> 的包装
- 在 Rust 看来，字符串有三种形式：
  - 字节： Bytes
    - `for x in s.bytes()` --- 进行遍历
  - Unicode 标量值： Scalar Values
    - 对一个 String 类型进行 **len()** 的调用，其返回值是根据该字符串的 Unicode 标量值来计算的
    - `for x in s.chars()` --- 进行遍历
  - 字形簇： Grapheme Clusters
    - Rust 没有进行包装，需要使用第三方库进行遍历 

### 切割 String

---

- 可以使用 **[]** 和 一个范围来创建字符串的切片
- 当切割字符串时，必须严格按照字符串的字符边界进行切割，不然程序会 panic

## 8.3 HashMap

- HashMap 的数据存放在 Heap 上
- HashMap 是同构的，即：所有的 key 和 Value 必须是同一种类型


### 创建 HashMap 

---

- 创建空的 `HashMap::new()` 函数，添加数据 `insert()` 方法
- 使用 `collect()` 方法，在元素类型是 Tuple 的 Vector 上使用 collect 方法，则可以组建一个 HashMap，但要求 Tuple 有两个值，一个作为 Key，一个作为 value
  - 因为 collect 方法可以把数据整合成很多种集合类型，所以需要手动指定 **返回值的类型** 

### HashMap 和所有权

---

- 对于实现了 Copy trait 的类型，值会被复制到 HashMap 中
- 对于拥有所有权的值，值会被移动，所有权会转移给 HashMap，如果将值的引用插入到 HashMap，值本身不会移动，**但在 HashMap 有效的期间，被引用的值必须保持有效**

### 访问 HashMap 的值

---

- 例子

### 遍历 HashMap

---

- 例子

### 更新 HashMap 的值

---

- 例子

### Hash 函数 

---

- 默认情况下，HashMap 使用加密功能强大的 Hash 函数，可以抵抗拒绝服务（DoS）攻击，不是可用的最快的 Hash 算法，但具有更好安全性
- 可以制定不同的 hasher 来切换到另一个函数
  - hasher 是实现 BuildHasher trait 的类型 
  - （该内容需要自行找资料，视频没有详细讲解）

# No.9 错误处理

---

- Rust 会在编译阶段提示错误，要求程序员在该阶段进行处理，所以 Rust 较为可靠
- 错误分为：**可恢复** 和 **不可恢复**
  - 可恢复：例如文件未找到，可再次尝试
  - 不可恢复：bug，例如：数组访问越界等
- 大多数语言是使用异常机制进行处理，但 Rust 没有类似异常的机制，它提供了
  - 可恢复错误：`Result<T,E>`
  - 不可恢复错误： `panic!` 宏

## 9.1 不可恢复的错误与 panic!

- 当 panic! 执行的过程:
  - 程序会打印一个错误信息
  - 展开（unwind）、清理调用栈（Stack）
  - 退出程序
- 默认的情况下，当 panic 发生时，程序会展开调用栈（工作量巨大），Rust 会沿着调用栈进行回溯，清理掉每个遇到的函数中的数据
- 程序员可以通过 toml 文件，设置为 **立即终止调用栈**，即：不进行清理，直接停止程序（内存的清理工作将由 OS 进行）
  - 在 **Cargo.toml** 中适当的 **profile** 部分设置 `panic = 'abort'`

### 使用 panic! 产生回溯信息

---

- 通过设置环境变量 `RUST_BACKTRACE=1` 获取回溯信息
- 通过设置环境变量 `RUST_BACKTRACE=full` 获取更加详细的回溯信息

## 9.2 Result 与可恢复的错误

### Ruslt 枚举

---

```rust
enum Result<T, E> {
  Ok(T),
  Err(E),
}
```

- T: 操作成功的情况下，Ok 变体里返回的数据的类型
- E: 操作失败的情况下，Err 变体里返回的错误的类型
- 在 Result<T, E> 有很多方法都是可以接收闭包（closure）作为参数，使用这些方法可以让代码更加简洁
- `unwrap` 方法，其作用相当于一个 match 表达式的一个快捷方法，如果 Result 结果是 Ok 则返回 Ok 里面的值；如果 Result 结果是 Err 则调用 panic! 宏
  - 该方法的错误信息不能自定义 
- `expect` 方法与 `unwrap‵ 方法类似，但可指定错误信息

### 传播错误

---

- 当返回值是 Result<T, E> 时，就可以传播错误
- **?** 运算符是用于传播错误的一种快捷方式，它只能用于返回 Result 类型的函数
  - 如果 Result 是 Ok：Ok中的值就是表达式的结果，然后 继续执行程序
  - 如果 Result 是 Err：Err就是 **整个函数** 的返回值，就像使用了 return
  - ? 运算符会隐式的调用 from 函数，它所接收到的错误类型会被转化为当前函数返回类型所定义的错误类型
    - Trait std::convert::From 上的 from 函数，用于错误之间的转换 
    - 只有每个错误类型都实现s了转换为所返回的错误类型的 from 函数，才可以使用 ? 
- 如果要在 main 函数中使用 ？ 运算符，则需要修改 main 函数的返回值类型
  - `fn main() -> Result<(), Box<dyn Error>>` ，其中 **Box<dyn Error>** 需要使用 **use std::error::Error** 才可以，它是一个 trait 对象，可简单理解为“任何可能的错误类型”

## 9.3 什么时候应该使用 panic!

---

- 总体原则：**在定义一个可能失败的函数时，优先考虑返回 Result，否则就 panic!**
- [视频链接](https://www.bilibili.com/video/BV1hp4y1k7SV?p=42&spm_id_from=pageDriver&vd_source=acd7a7a6ca9b19c883482c0d52f771d5)

# No.10 泛型、Trait、生命周期

## 10.1 提取函数以消除重复代码

- 消除重复代码，最好的方式是将相同的逻辑，抽象成一个函数，后续只需要使用这个函数就可以完成该功能

## 10.2 泛型

- 泛型定义：`fn largest<T>(list:&[T]) -> T{...}`
- 类型参数：
  - 通常是一个字母 
  - 驼峰命名法
  - 一般使用 T：type 缩写

### 函数定义中的泛型

---

- 泛型函数： 参数类型，返回值类型

### struct 定义中的泛型

---

- 如果 struct 是只有一个泛型类型，则字段必须全部都是一样；如果是多个泛型类型，则字段可不同
- 例子

### enum 定义中的泛型

---

- 枚举的泛型一般用于变体中，且是变体带数据类型的，例如：`Option<T>`,`Result<T, E>`
- 例子

### 方法定义中泛型

---

- `impl<T> Point<T> { fn x(&self) -> &T{...}}`
  - 注意：当使用泛型方法后，如果再在内部定义一个指明类型的方法，那么该类型的方法列表中是看不到泛型方法的 
- 例子
- struct 里的泛型类型参数可以和方法的泛型类型参数不同

### struct 泛型类型参数与方法的泛型类型参数不一样

---

- 例子

### 泛型代码的性能

---

- 使用泛型的代码和使用具体类型的代码运行速度是一样的，因为 Rust 使用 **单态化**（在编译时将泛型替换为具体类型的过程）


## 10.3 Trait

- Trait 告诉 Rust 编译器：某种类型具有哪些并且可以与其他类型共享的功能
- Trait：抽象的定义共享行为
- Trait Bound(约束)：泛型类型参数指定为实现了特定行为的类型
- Trait 与其它语言的接口（interface）类似，但有些区别

### 定义一个 Trait

---

- Trait 的定义：把方法签名放在一起，来定义实现某种目的所必需的一组行为
- 关键字：**trait**
- 只有方法签名，没有具体实现
- trait 可以有多个方法，每个方法签名占一行，以 **;** 结尾
- 实现该 trait 的类型必须提供具体的方法实现

### 在类型上实现 Trait

---

- 与为类型实现方法类似，不同之处在于需要加上 trait 名，`impl Trait_Name for TypeName {...}`
- 例子

### 实现 Trait 的约束

---

- 可以在某个类型上实现某个 trait 的前提条件是：这个 **类型** 或这个 **trait** 是在 **本地 carte** 里定义的
- 无法为外部类型来实现外部的 trait，例如：无法为标准库的 String 类型实现标准库的 Display 这个 trait

### 默认实现

---

- 例子
- 默认方法可以调用 trait 的其他方法，即使这些方法没有默认实现
- <font color=red>注意：无法从方法的重写实现里面调用默认的实现</font>

### Trait 作为参数

---

- 使用 impl trait 语法：适用于简单情况
- 使用 Trait Bound 语法：可用于复杂情况
  - impl Trait 语法 是 Trait Bound 的语法糖
- 使用 **+** 指定多个 Trait Bound

### 实现 Trait 作为返回类型

---

- 使用 impl trait 语法
  - <font color=red>注意： impl trait 只能返回确定的 **同一种类型**，返回可能不同类型的代码会报错</font>

### 使用 Trait Bound 的例子

---

- 修复 largest 函数的例子
  - Note: 在进行引用比较时，Rust 编译器会自动的进行解引用
  - Note: println!() 函数中，如果参数是一个引用，Rust 编译器会自动进行解引用

### 使用 Trait bound 有条件的实现方法

---

- 在使用泛型类型参数的 impl 块上使用 Trait Bound，我们可以有条件的为实现了特定 Trait 的类型来实现方法
- 也可以为实现了其他 Trait 的任意类型有条件的实现某个 Trait，为满足 Trait Bound 的所有类型上实现 Trait 叫做覆盖实现（blanket implementations）

## 10.4 生命周期（life cycle）

- Rust 的每个引用都有自己的生命周期
- 生命周期：引用保持有效的作用域
- 大多数情况：生命周期是隐式的、可被推断的
- 当引用的生命周期可能以不同的方式互相关联时，需要手动标注生命周期

### 生命周期 - 避免悬垂引用（dangling reference）

---

- 主要目标：避免悬垂引用

### 借用检查器

---

- 比较作用域，从而判断所有的借用是否合法

### 函数中的泛型生命周期

---

- 例子

### 生命周期标注语法

---

- 生命周期的标注不会改变引用的寿命周期长度，当指定了泛型生命周期参数，函数可以接收带有任何生命周期的引用。即：**描述了多个引用的生命周期期间的关系，但不影响生命周期**
- 生命周期参数名：
  - 以 **'** 开头，通常全小写且非常短，一般情况下，使用的是  `'a`
- 生命周期标注的位置：
  - 在引用符号 `&` 后面，使用空格将标注和引用类型分开
- 例子：
  - `&ℹ32` --> 一个引用
  - `&'a i32` --> 带有显式生命周期的引用
  - `&'a mut i32` --> 带有显式生命周期的可变引用

### 函数签名中的生命周期标注

---

- 泛型生命周期参数声明在：函数名和参数列表之间的 **<>** 里

### 深入理解生命周期

---

- 指定生命周期参数的方式依赖于函数所做的事情，例子
- 从函数返回引用时，返回类型的生命周期参数需要与其中一个参数的生命周期匹配，如果返回的引用没有指向任何参数，那么它就是悬垂引用，例子 
- 生命周期实际上就是 Rust 用来关联函数的不同参数引用和返回值引用的生命周期的，当 Rust 获取了足够的信息，就可以维护内存安全，从而避免不安全的操作

### Struct 定义中的生命周期标注

---

- 如果结构体的字段类型是引用的话，则必须在每个引用上添加生命周期标注
- 例子

### 生命周期的省略

---

- 在 Rust 引用分析中所编入的模式称为 **生命周期省略规则**
- 如果代码符合生命周期省略规则，则无需显式标注生命周期

#### 输入、输出生命周期

- 生命周期出现在函数/方法的参数 -- **输入生命周期**
- 生命周期出现在函数/方法返回值 -- **输出生命周期**

#### 生命周期省略的三个规则

1. 每个引用类型的参数都有自己的生命周期，即：每个参数都有各自的生命周期参数，应用于 **输入生命周期**
2. 如果只有一个输入生命周期参数，那么该生命周期被赋给所有的输出生命周期参数，应用于 **输出生命周期**
3. 如果有多个输入生命周期参数，但其中一个是`&self` 或 `&mut self`，那么 self 的生命周期会被赋给所有的输出生命周期参数， 应用于 **输出生命周期**

```rust
/// 生命周期省略的三个规则 - 例子
// 例子 1
// fn first_word(s: &str) -> &str {...}
// 应用规则 1
// fn first_word<'a> (s: &'a str) -> &str {...}
// 应用规则 2
// fn first_word<'a> (s: &'a str) -> &'a str {...}
//
// 例子 2
// fn longest(x: &str, y:&str) ->&str {...}
// 应用规则 1
// fn longest<'a, 'b> (x: &'a str, y:&;b str) ->&str {...}
// 应用规则 2，编译器发现无法为返回值赋予生命周期参数，编译器就报错
// fn longest<'a, 'b> (x: &'a str, y:&;b str) ->&str {...} 
```

### 方法定义中的生命周期标注

---

- 在 struct 上使用生命周期实现方法，语法和泛型参数的语法是一样的
- 在哪声明和使用生命周期参数，依赖于生命周期参数是否和 **字段、方法的参数或返回值** 有关
- struct 字段的生命周期名：
  - 在 impl 后声明
  - 在 struct 名后使用
  - 这些生命周期是 struct 类型的一部分
- impl 块内的方法签名中：
  - **引用** 必须绑定于 struct 字段引用的生命周期，或者 **引用** 是独立的也可以
  - 生命周期省略规则经常使得方法中的生命周期标注不是必须的

### 静态生命周期

---

- `'static` 是一个特殊的生命周期，它与程序共同存活
- 不建议使用或尽量少的使用静态生命周期

### 泛型参数类型、Trait Bound、生命周期

---

- 综合例子

# 11 编写自动化测试

## 11.1 编写和运行测试

- 测试就是执行一个特定功能的函数，其功能主要是验证非测试代码的功能是否与预期一致
- 测试函数通常执行 3 个操作（简称：3A 操作）
  - 准备数据/状态
  - 运行被测试的代码
  - 断言（Assert）结果

### 解剖测试函数

---

- 测试函数需要使用 **test** 属性（attribute）
  - Attribute 就是一段 Rust 代码的元数据，不会修改代码的逻辑，只是对代码的修饰或标注
- 将一个函数标记为测试函数

```rust
#[test]
fn fnc_name() {}
```

### 运行测试

--- 

- `cargo test` 运行 **所有** 测试函数，会直接运行标注了 **test** 的函数，并报告其运行是否成功
- 使用 `cargo new project_name --lib` 创建 library 项目的时候，会直接生成一个 test module，里面有一个 test 函数
- 定义测试模块

```rust
#[cfg(test)]
mod tests {
  
  #[test]
  fn it_works() {
    assert_eq!(2+2, 4);
  }
}
```

### 测试失败

--- 

- 测试函数 panic 代表失败
- 每个测试运行在一个新线程，当主线程看见某个测试线程挂掉了，则表示那个测试标记为失败了

## 11.2 断言（Assert）

### 使用 `assert!` 宏检查测试结果

---

- 检查某个状态是否为 true，如果该状态为 false，则会调用 panic!
- 使用 `assert_eq!()` 和 `assert_ne!` 可以测试参数的相等性
  - 如果直接使用 `assert!(param_1 == param_2)`，这种只会打印出结果是否相等，而使用上面两个宏，则会在失败时，将参数的值也打印出来
  - 因为要打印参数，使用的是 debug 格式，要求参数实现了 **PartialEq 和 Debug Traits

## 11.3 添加自定义错误信息

---

- 添加自定义错误信息的位置是原参数的末尾的后面，即 `assert!(param, diy_msg)`, `assert_eq!(param1, param2, diy_msg)`, `assert_ne!(param1, param2, diy_msg)`


## 11.4 用 should_panic 检查恐慌

- 使用 `#[should_panic]` 来检查是否会发生错误，如果函数发生 panic，则测试通过；否则测试失败
- 添加 **expected** 参数，从而让 shoud_panic 更加精确

## 11.5 在测试中使用 Result<T, E>

- 返回 Ok，测试通过；返回 Err, 测试失败
- <font color=red> 注意：不要在使用 **Result** 编写的测试上标注 `#[should_panic]` </font>

## 11.6 控制测试运行：并行和连续执行测试

- `cargo test -- --test-threads=1` --> 不想以并行方式运行测试，或相对线程数量进行细粒度控制
- `cargo test -- --show-output` --> 让 Rust 的 test 库捕获所有打印到标准输出的内容

## 11.7 按测试名称运行测试

- `cargo test test_fnc_name` --> 运行指定的测试函数
  - 以上只能运行单个测试，如果要指定运行多个测试函数，可以将多个测试函数名共有的部分替换掉上面的 **test_fnc_name** 就可以了 

## 11.8 忽略测试

- 使用 `#[ignore]` 属性来忽略特定的测试函数
- 当需要运行被忽略的测试函数，可以使用 `cargo test -- --ignored`

## 11.9 测试组织：单元测试

- 单元测试会将所有需要测试的代码都存放到一个单独的文件中，并在文件中创建一个 test module，并使用 `#[cfg(test)]` 标注

### 测试分类

---

- Rust 的测试共有两类：单元测试和集成测试
- 单元测试：
  - 小、专注
  - 一次对一个模块进行隔离测试
  - 可测试 **private** 接口
- 集成测试：
  - 在库外部，和其他外部代码一样使用代码
  - 只能使用 **public** 接口
  - 可能在每个测试中使用到多个模块

### #[cfg(test)] 标注

- 使用了该标注，只有在运行 `cargo test` 才会进行编译和运行代码
  - 集成测试在不同的目录，它不需要该标注
- **cfg(test)** 是用来告诉 Rust，只有在使用 test 配置(`cargo test`)时，才编译和运行代码
- 如果在测试模块中，添加了一个函数，但不使用 `#[test]` 标注，则该函数只会被编译不会执行

### 测试私有函数

---

- 例子

## 11.10 集成测试

- 在 Rust 里，集成测试完全位于被测试库的外部，目的是测试被测试库的多个部分是否能正确的一起工作
- 集成测试仅能使用在 library crate 中，不能使用在 binary crate 中，即：不能使用在只含有 main.rs 的 crate 中


### tests 目录

---

- tests 目录与 src 目录同级
- tests 目录下的每个测试文件都是单独的一个 crate，所以需要将被测试库导入
- 对于 tests 目录， Rust 默认只会在 `cargo test` 中编译执行，所以不需要 `#[cfg(test)]`

### 运行指定的集成测试

---

- `cargo test fnc_name` --> 运行一个特定的集成测试
- `cargo test --test test_file_name` --> 运行某个测试文件内的所有测试

### 集成测试中的子模块

---

- 在 tests 目录下面的子目录，则不会被 Rust 视为一个测试文件
  - 具体操作：目录为模块名，里面创建一个 **mod.rs** 文件，文件内部写入代码

### 针对 binary crate 的集成测试

---

- 因为只有 library crate 才能暴露函数给其它 crate 用，所以要在 binary crate 中创建 lib.rs
- binary crate 是独立运行的，无法把 main.rs 的函数导入作用域

# No.12 项目实例： 命令行程序

- 内容：
  1. 接受命令行参数
  2. 读取文件
  3. 重构：改进模块和错误处理
  4. 使用 TDD（测试驱动开发）开发库功能
  5. 使用环境变量
  6. 将错误消息写入标准错误而不是标准输出

## 12.1 接受命令行参数

- 目标：`cargo run search_content file_name` -> 从 file_name 中 找到 search_content 的东西
- 接收命令行参数，可以使用 `std::env::arg()`，获取到后，需要使用 `collect` 函数将其转化成集合

## 12.2 读取文件

- 读取文件内容，可以使用 `std::fs::read_to_string()`
- `cargo run xxx 123.txt` 是在项目目录下读取 123.txt 文件

## 12.3 重构：改进模块和错误处理

### 二进制程序关注点分离的指导性原则

---

- 将程序拆分为 **main.rs** 和 **lib.rs**，将业务逻辑放入 **lib.rs**
- 当命令行解析逻辑较少时，将它放在 **main.rs** 也可以
- 当命令行解析逻辑变复杂时，需要将它从 **main.rs** 提取到 **lib.rs**

## 12.4 使用 TDD（测试驱动开发）开发库功能

### 测试驱动开发（Test-Driven Development)

---

1. 编写一个会失败的测试，运行该测试，确保它是按照预期的原因失败
2. 编写或修改刚好足够的代码，让新测试通过
3. 重构刚刚添加或修改的代码，确保测试会始终通过
4. 返回步骤 1，继续

## 12.5 使用环境变量

- `env::var()` 判断是否有指定的环境变量（环境变量的值默认是大写的）

## 12.6 将错误消息写入标准错误而不是标准输出

### 标准输出 VS 标准错误

---

- 标准输出：stdout，函数宏：`println!()`
- 标准错误：stderr，函数宏：`eprintln!()`

# No.13 函数式语言特性: 迭代器和闭包

1. 闭包（closure）
2. 迭代器（iterators）
3. 优化改善 12 章的实例项目
4. 讨论闭包和迭代器的运行时性能

## 13.1 闭包

### 使用闭包创建抽象行为

---

- 闭包：可以捕获其所在环境的匿名函数
  - 是一个匿名函数
  - 可以保存为变量，或者作为参数
  - 可以在一个地方创建闭包，然后在另一个上下文中调用闭包来完成运算
  - 可以从其定义的作用域捕获值
- 定义：
```rust
let closure = |param: u32| -> u32 {
  param
};
 ```
  
### 闭包的类型推断和标注

---

- 闭包不要求标注参数和返回值的类型，因为闭包不会暴露在使用者的代码中
- 闭包通常都是很短小（<font color=red>如果一个闭包很复杂，则表示该闭包应该被转化为函数</font>），只在狭小的上下文中工作，编译器一般情况下都可以推断出类型
- 可以手动添加类型标注
- <font color=red>注意：闭包的定义最终只会为参数、返回值推断出 **唯一** 具体的类型</font>

#### 函数和闭包的定义语法

```rust
fn  add_one_v1      (x: u32)    -> u32 { x + 1}     // 函数
let add_one_v2 =    |x: u32|    -> u32 { x + 1};    // 闭包，手动添加类型标注
let add_one_v3 =    |x|                { x + 1};    // 闭包，由编译器推断类型
let add_one_v4 =    |x|                  x + 1 ;    // 闭包，由于函数体内部只有一个表达式，所以可以将花括号进行省略
```

### 记忆化（memoization）或延迟计算（lazy evaluation）

---

- 创建一个结构体，其持有闭包和调用结果，当调用结果为空值时，将调用闭包，并将结果保存到调用结果字段中

#### 如何让结构体持有闭包

- 结构体的定义是需要指导所有字段的类型，所以需要指明闭包的类型
  - 指明闭包的类型，需要使用到泛型和 Trait Bound
- <font color="red"> 每个闭包实例都有自己唯一的匿名类型，即使两个闭包签名完全一样 </font>
- 要使用 trait bound，则必须实现 **Fn traits**， 此由标准库提供，所有的闭包都至少实现了以下 trait 之一：
  - Fn
  - FnMut
  - FnOnce 

```rust
struct Cacher<T>
    where T: Fn(u32) -> u32
{
    calculation: T,
    value: Option<u32>,
}

impl<T> Cacher<T>
    where T: Fn(u32) -> u32
{
    fn new(calculation: T) -> Cacher<T> {
        Cacher {
            calculation,
            value: None,
        }
    }

    // 这个方法的含义就是当 value 为 None 时，将会调用闭包，并将结果保存到 value 和 将结果返回
    fn value(&mut self, arg: u32) -> u32 {
        match self.value {
            Some(v) => v,
            None => {
                let v = (self.calculation)(arg);
                self.value = Some(v);
                v
            }
        }
    }
}
```

### 闭包从所在环境捕获值

#### 闭包可以捕获他们所在的环境

- 闭包可以访问定义它的作用域内的变量，而普通函数不可以。但是会额外产生的内存开销

```rust
fn main() {
  let x = 5;
  let equal_x = |z| z == x;
  let y = 4;
  assert!(equal_x(y));
}
```

#### 闭包从所在环境捕获值的方式
- 与函数获得参数的三种方式一样：
  1. 取得所有权：FnOnce
  2. 可变借用：FnMut
  3. 不可变借用：Fn
- 创建闭包时，通过闭包对环境值的使用，Rust 可以推断出具体使用哪个 trait：
  - 所有的闭包都实现了 FnOnce
  - 没有移动捕获变量的实现了 FnMut
  - 无需可变访问捕获变量的闭包实现了 Fn
  - <font color=red> 所有实现了 Fn 的都实现了 FnMut，所有实现了 FnMut 的都实现了 FnOnce </font>

#### move 关键字
- 在参数列表前使用 move 关键字，可以强制闭包取得它所使用的环境值的所有权
  - 当将闭包传递给新线程，以移动数据，使其归新线程所有时，此技术最为有用

```rust
fn main() {
  let x = vec![1,2,3];
  let equal_x = move |z| z == x;
  println!("can't use x here: {:?}", x); // 这里会报错，x 的所有权被移动到闭包内

  let y = vec![1,2,3];
  assert!(equal_x(y));
}
```
- <font color=red> 当要使用闭包 trait 时，优先使用 Fn，如果要使用 FnMut 或者 FnOnce 时，编译器会有提示 </font>

## 13.2 迭代器

- 迭代器模式：对一系列项执行某些任务
- 迭代器负责：
  - 遍历每个项
  - 确定序列（遍历）何时完成
- Rust 迭代器：
  - 懒惰的：除非调用消费迭代器的方法，否则迭代器本身没有任何效果

```rust
fn main() {
  let v1 = vec![1,2,3];
  let v1_iter = v1.iter(); // 产生一个迭代器，没有对 v1_iter 进行操作时，是不产生开销的

  // 当执行下面代码时，迭代器才会产生开销
  for val in v1_iter {
    println!("Got: {}", val);
  }
}
```

### 迭代器方法区别
- iter(): 在不可变引用上创建迭代器
- into_iter(): 创建的迭代器会获得所有权
- iter_mut(): 迭代可变的引用

### 消耗迭代器的方法
- 实现 iterator trait 时必须实现 next 方法的原因之一在于内部会调用 next 方法
- 调用 next 的方法叫做“消耗型适配器”，因为调用它们会把迭代器消耗尽

```rust
fn main() {
  let v1 = vec![1,2,3];
  let v1_iter = v1.iter();
  let total: i32 = v1_iter.sum();// sum 方法会将 v1_iter 的项都消耗掉
  assert_eq!(total, 6);
}
```

### 产生其它迭代器的方法
- 定义在 Iterator trait 上的另外一些方法叫做“迭代器适配器”，把迭代器转换为不同种类的迭代器
- 可以通过链式调用使用多个迭代器适配器来执行复杂的操作

```rust
fn main() {
  let v1: Vec<i32> = vec![1,2,3];
  // 如果只是下面的表达式，实际上不会进行+1操作，因为 rust 的迭代器是懒惰的
  // v1.iter().map(|x| x+1);

  let v2: Vec<_> = v1.iter().map(|x| x+1).collect();
}
```

### 使用闭包捕获环境（配合迭代器）
- filter 方法：
  - 接收一个闭包，在遍历迭代器的每个元素时，该闭包返回 bool 类型
  - 如果返回 true：当前元素将会包含在 filter 产生的迭代器中
  - 如果返回 false：当前元素将不会包含在 filter 产生的迭代器中

```rust
#[derive(PartialEq, Debug)]
struct Shoe {
  size: u32,
  style: String,
}

fn shoes_in_my_size(shoes: Vec<Shoe>, shoe_size: u32) -> Vec<Shoe> {
  shoes.into_iter() // 创建一个获得所有权的迭代器
    .filter(
      |x| x.size == shoe_size
    )
    .collect()
}
```

### 创建自定义的迭代器
- 实现 next 方法即可

```rust
struct Counter {
  count: u32,
}

impl Counter {
  fn new() -> Counter {
    Counter {
      count: 0
    }
  }
}

impl Iterator for Counter {
  type Item = u32;
  // Self:Item：可以暂时理解为 u32 类型
  fn next(&mut self) -> Option<Self:Item> {
    if self.count < 5 {
      self.count += 1;
      Some(self.count)
    }
    else {
      None
    }
  }
}

fn main() {
  let sum: u32 = Counter::new()
    .zip(Counter::new().skip(1)) // 链接一个新的略过 1 个元素的 Counter
    .map( |(a,b)| a * b) // 映射为元组类型，进行两值相乘
    .filter( |x| x % 3 == 0) // 只保留可以被 3 整除的
    .sum() // 将所有项的和进行相加

  assert_eq!(18, sum);
}
```

# No.14 cargo, catres.io
1. 通过 release profile 来自定义构建
2. 在 https://crates.io/ 上发布库
3. 通过 workspaces 组织大工程
4. 从 https://crates.io/ 来安装库
5. 使用自定义命令扩展 cargo

## 14.1 通过 release profile 来自定义构建
- 每个 profile 的配置都是独立于其它的 profile
- Cargo 主要的两个 profile:
  - dev profile: 适用于开发，cargo build
  - release profile：适用于发布，cargo build --release

### 自定义 profile
- 针对每个 profile，Cargo 都提供了默认的配置
- 如果项自定义 xxx profile 的配置：
  - 在 Cargo.toml 里添加 [profile.xxx] 区域，在里面覆盖默认配置的子集

```toml
# 想覆盖默认的 dev profile
[profile.dev]
opt-level = 0 # 代码优化等级

# 想覆盖i默认的 release profile
[profile.release]
opt-level = 3
```

## 14.5 Cargo 工作空间
- cargo 工作空间：帮助管理多个相互关联切需要协同开发的 crate
- cargo 工作空间是一套共享同一个 Cargo.lock 和输出文件夹的包

### 创建工作空间
1. 手动创建工作空间的目录
2. 手动创建 Cargo.toml 文件
```toml
[workspace]
[workspace]

members = [
    "demo", # 这里写的是 crate 的目录名
    "minigrep",
    "old_minigrep"
]
```

# No.15 智能指针

- 智能指针是一些行为和指针类似，有额外的元数据和功能的数据结构
1. 介绍标准库中常见的智能指针类型
   - Box<T>: 在 heap 内存上分配值
   - Rc<T>: 启用多重所有权的引用计数类型
   - Ref<T> 和 RefMut<T>，通过 RefCell<T> 访问: 在运行时而不是编译时强制借用规则的类型
2. 内部可变模式: 不可变类型暴露出可修改其内部值的 API
3. 引用循环: 它们如何泄漏内存，以及如何防止其发生

## 引用计数智能指针类型
- 通过记录所有者的数量，是一份数据被多个所有者同时持有
- 并在没有任何所有者时，自动清理数据

### 引用和智能指针的其它不同
- 引用：只借用数据
- 智能指针：很多时候都拥有它所指向的数据

### 智能指针的例子
- String 和 Vec<T>

### 智能指针的实现
- 智能指针通常使用 struct 实现，并且实现了 Deref 和 Drop 两个 trait
  - Deref trait：允许智能指针 struct 的实例像引用一样使用
  - Drop trait：允许自定义智能指针实例走出作用域时自动释放的代码

## 15.1 使用 Box<T> 来指向 Heap 上的数据
- Box<T> 是最简单的智能指针：
  - 允许你在 heap 上存储数据，而不是 stack
  - stack 上是指向 heap 数据的指针
  - 没有性能开销
  - 没有其他额外功能
  - 实现了 Deref 和 Drop trait


### Box<T> 的常用场景
1. 在编译时，某类型的大小无法确定，但在使用该类型时，上下文却需要知道它的确切大小
2. 当你有大量数据，想移交所有权，但需要确保在操作时，数据不会被复制
3. 时用某个值时，你只关心她是否实现了特定的 trait，而不关心它的具体类型

```rust
fn main(){
  let b = Box::new(5);
  println!("b = {}", b);
}
```

### 使用 Box 赋能递归类型
- 在编译时，Rust 需要知道一个类型所占的空间大小，而递归类型的大小无法在编译时确定
