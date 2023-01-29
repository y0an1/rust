/// 创建 Box<T>
#[allow(unused)]
mod demo1 {
    pub fn main() {
        let b = Box::new(5);
        println!("b: {}", b); // b: 5
    } // 当调用 drop 函数时，会清理 b 的 stack 的指针和 heap 的内存数据
}

/// 用 Box<T> 来解决 Cons list 问题
#[allow(unused)]
mod demo2 {
    use crate::demo::r#box::demo2::List::{Cons, Nil};
    // 因为递归，导致 Rust 无法在编译期确定其内存大小，无法编译
    // enum List{
    //     Cons(i32, List),
    //     Nil,
    // }

    #[derive(Debug)]
    // 使用 Box 来进行包装，因为 Box 类型，Rust 可以确定大小
    enum List {
        Cons(i32, Box<List>),
        Nil,
    }

    pub fn main() {
        let list = Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Nil))))));
        println!("list: {:#?}", list);
    }
}

pub fn main() {
    // demo1::main();
    demo2::main();
}
