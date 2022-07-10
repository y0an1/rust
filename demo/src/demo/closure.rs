

/// 闭包的定义最终只会为参数、返回值推断出 **唯一** 具体的类型
mod demo1 {
    pub fn main() {
        let closure = |x| x;    // 在闭包被定义时，编译器无法推导出闭包的类型
        let s = closure(String::from("hello")); // 在调用闭包时，传入了 String 类型，此时，编译器就可以推导出闭包类型了
        // let n = closure(5); // 当一个闭包只能处理一个类型，其不支持泛型处理，即：当编译器为闭包推导出类型后，该闭包只能处理该类型的数据
        // error[E0308]: mismatched types
        //  --> src\demo\closure.rs:8:25
        //   |
        // 8 |         let n = closure(5); //
        //   |                         ^- help: try using a conversion method: `.to_string()`
        //   |                         |
        //   |                         expected struct `String`, found integer
    }
}

pub fn main() {
    demo1::main();
}