/// 建立 module
#[allow(unused)]
mod front_of_house {
    mod hosting {
        fn add_to_wait_list() {}

        fn seat_at_table() {}
    }

    mod serving {
        fn take_order() {}

        fn serve_order() {}

        fn take_payment() {}
    }
}

/// 将 demo 模块下的所有东西导入到该文件
mod demo;

/// 使用通配符导入 demo 模块下所有的东西
// use crate::demo::*;

/// 使用嵌套路径来导入
use crate::demo::{
    borrow, closure, ctrl_stream, data_type, error, generics, hashmap, hello_rust, life_cycle,
    memleak, object, owner, r#box, r#enum, r#extern, r#match, r#struct, r#trait, r#unsafe, rc,
    refcell, ret_value, slice, string, thread, variables, vector,
};

pub fn main() {
    hello_rust::main();
    data_type::main(); // 数据类型
    variables::main(); // 变量
    ctrl_stream::main(); // 控制流
    ret_value::main(); // 返回值
    owner::main(); // 所有权
    borrow::main(); // 引用
    slice::main(); // 切片
    r#struct::main(); // 结构体
    r#enum::main(); // 枚举
    r#match::main(); // 控制流匹配运算符 match
    vector::main(); // 动态数组
    string::main(); // 字符串
    hashmap::main(); // hashmap
    error::main(); // 错误处理
    generics::main(); // 泛型
    r#trait::main(); // 接口
    life_cycle::main(); // 生命周期
    closure::main(); // 闭包
    r#box::main(); // Box<T>
    rc::main(); // Rc<T>
    refcell::main(); // RefCell<T>
    memleak::main(); // 循环引用内存泄漏
    thread::main();
    object::main();
    r#unsafe::main();
    r#extern::main();
}
