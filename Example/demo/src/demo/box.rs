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

/// 定义自己的智能指针
#[allow(unused)]
mod demo3 {
    use std::ops::Deref;

    pub(super) struct MyBox<T>(T);

    impl<T> MyBox<T> {
        pub(super) fn new(x: T) -> MyBox<T> {
            MyBox(x)
        }
    }

    // 实现 Deref trait
    impl<T> Deref for MyBox<T> {
        type Target = T;

        fn deref(&self) -> &T {
            &self.0
        }
    }

    pub fn main() {
        let x = 5;
        // let y = Box::new(x);
        let y = MyBox::new(x);

        assert_eq!(x, 5);
        assert_eq!(x, *y); // 编译器会自动将 *y 展开成 *(y.deref())
    }
}

/// Deref Coercion
#[allow(unused)]
mod demo4 {
    use super::demo3::MyBox;

    fn hello(name: &str) {
        println!("Hello, {}", name);
    }

    pub fn main() {
        let m = MyBox::new(String::from("Rust"));

        // &m => &MyBox<String>
        // 编译器会自动调用 MyBox::deref 方法来进行解引用
        // &String => String
        // 编译器会自动调用 String::deref 方法来进行解引用，返回的是 &str 类型
        hello(&m);
        hello("Rust");
    }
}

/// Drop trait
#[allow(unused)]
mod demo5 {
    struct CustomSmartPointer {
        data: String,
    }

    impl Drop for CustomSmartPointer {
        fn drop(&mut self) {
            println!("Dropping CustomSmartPointer with dat `{}`!", self.data);
        }
    }

    pub fn main() {
        let c = CustomSmartPointer {
            data: String::from("my stuff"),
        };

        drop(c); // 直接调用 std::mem::drop 来释放资源

        let d = CustomSmartPointer{
            data: String::from("other stuff")
        };
        println!("CustomSmartPointers created.")
    }// 先调用 d 的 drop 方法，然后是 c 的 drop 方法
}

pub fn main() {
    // demo1::main();
    // demo2::main();
    // demo3::main();
    // demo4::main();
    // demo5::main();
}
