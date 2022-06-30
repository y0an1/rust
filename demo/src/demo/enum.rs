
#[allow(unused)]
mod demo1 {
    /// 声明一个枚举类型
    #[derive(Debug)]
    pub enum IpAddrKind {
        V4, // 枚举变体
        V6,
    }

    pub fn main() {
        let v4 = IpAddrKind::V4;
        let v6 = IpAddrKind::V6;
        println!("v4: {:?}, v6:{:?}", v4, v6);
    }
}

#[allow(unused)]
mod demo2 {
    use crate::r#enum::demo1::IpAddrKind;

    /// 枚举作为 struct 字段类型
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


#[allow(unused)]
mod demo3 {
    /// 将数据附加到枚举变体中
    #[derive(Debug)]
    enum IpAddr {
        V4(u8, u8, u8, u8),
        V6(String),
    }

    pub fn main() {
        let home = IpAddr::V4(127, 0, 0, 1);
        println!("home: {:?}", home);

        let lookback = IpAddr::V6(String::from(":1"));
        println!("lookback: {:?}", lookback);
    }
}

#[allow(unused)]
mod demo4 {
    /// 使用Option<T>
    pub fn main() {
        let some_number = Some(5);
        let some_string = Some("A String");

        // let absent_number = None;    // 当使用 Option<T>::None 时，如果不指定类型，编译器则无法推导出变量的类型
        let absent_number:Option<i32> = None;
    }
}

#[allow(unused)]
mod demo5 {
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
}