/// 声明一个枚举类型
#[allow(unused)]
mod demo1 {
    #[derive(Debug)]
    pub enum IpAddrKind {
        V4,
        // 枚举变体
        V6,
    }

    pub fn main() {
        let v4 = IpAddrKind::V4;
        let v6 = IpAddrKind::V6;
        println!("v4: {:?}, v6:{:?}", v4, v6);
    }
}

/// 枚举作为 struct 字段类型
#[allow(unused)]
mod demo2 {
    use crate::r#enum::demo1::IpAddrKind;

    #[derive(Debug)]
    struct IpAddr {
        kind: IpAddrKind,
        address: String,
    }

    pub fn main() {
        let home = IpAddr {
            kind: IpAddrKind::V4,
            address: String::from("127.0.0.1"),
        };
        println!("home: {:?}", home);
        // home: IpAddr { kind: V4, address: "127.0.0.1" }

        let loopback = IpAddr {
            kind: IpAddrKind::V6,
            address: String::from(":1"),
        };
        println!("loopback: {:?}", loopback);
        // loopback: IpAddr { kind: V6, address: ":1" }
    }
}

/// 将数据附加到枚举变体中
#[allow(unused)]
mod demo3 {
    // 枚举的附加数据的类型可以是任意的
    #[derive(Debug)]
    enum IpAddr {
        V4(u8, u8, u8, u8),
        V6(String),
    }

    pub fn main() {
        let home = IpAddr::V4(127, 0, 0, 1);
        let loopback = IpAddr::V6(String::from(":1"));
        println!("home: {:?}", home);
        println!("loopback: {:?}", loopback);
    }
}

/// 枚举定义方法
mod demo4 {
    #[derive(Debug)]
    enum IpAddr {
        V4(u8, u8, u8, u8),
        V6(String),
    }

    impl IpAddr {
        fn show(&self) -> String {
            "show".to_string()
        }
    }

    pub fn main() {
        let home = IpAddr::V4(127, 0, 0, 1);
        let s = home.show();
        println!("{}", s);
    }
}

/// 使用 Option<T>
#[allow(unused)]
mod demo5 {
    pub fn main() {
        let some_number = Some(5);
        let some_string = Some("A String");
        let absent_number: Option<i32> = None; // 当使用 Option<T>::None 时，如果不指定类型，编译器则无法推导出变量的类型
    }
}

/// Option<T> 与 T 是不同类型，无法进行直接运算
#[allow(unused)]
mod demo6 {
    pub fn main() {
        let x = 5;
        let y = Some(6);
        // println!("{}", x + y);
        // error[E0277]: cannot add `Option<{integer}>` to `{integer}`
        // --> src/demo/enum/mod.rs:75:22
        //     |
        //     75 |     println!("{}", x + y);
        // |                      ^ no implementation for `{integer} + Option<{integer}>`
        // |
        // = help: the trait `Add<Option<{integer}>>` is not implemented for `{integer}`

        println!("{}", x + y.expect("fruits are healthy"));
    }
}

pub fn main() {
    // demo1::main();
    // demo2::main();
    // demo3::main();
    // demo4::main();
    // demo5::main();
    // demo6::main();
}