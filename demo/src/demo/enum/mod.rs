
/// 声明一个枚举类型
#[derive(Debug)]
#[allow(unused)]
enum IpAddrKind {
    V4, // 枚举变体
    V6,
}

#[allow(unused)]
fn demo1() {
    let v4 = IpAddrKind::V4;
    let v6 = IpAddrKind::V6;
    println!("v4: {:?}, v6:{:?}", v4, v6);
}

/// 枚举作为 struct 字段类型
#[derive(Debug)]
#[allow(unused)]
struct IpAddr {
    kind: IpAddrKind,
    address: String,
}

#[allow(unused)]
fn demo2() {
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

/// 将数据附加到枚举变体中
#[derive(Debug)]
#[allow(unused)]
enum IpAddr_ {
    V4(u8, u8, u8, u8),
    V6(String),
}

#[allow(unused)]
fn demo3() {
    let home = IpAddr_::V4(127, 0, 0, 1);
    println!("home: {:?}", home);
    // home: V4(127, 0, 0, 1)

    let lookback = IpAddr_::V6(String::from(":1"));
    println!("lookback: {:?}", lookback);
    // lookback: V6(":1")
}

/// 使用Option<T>
#[allow(unused)]
fn demo4() {
    let some_number = Some(5);
    let some_string = Some("A String");

    // let absent_number = None;    // 当使用 Option<T>::None 时，如果不指定类型，编译器则无法推导出变量的类型
    let absent_number:Option<i32> = None;
}

#[allow(unused)]
fn demo5() {
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

pub fn main() {
    // demo1();
    // demo2();
    // demo3();
    // demo4();
    // demo5();
}