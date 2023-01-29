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
    hello_rust, data_type, variables, ctrl_stream, ret_value, owner, borrow,
    slice, r#struct, r#enum, r#match, vector, string, hashmap, error, generics,
    r#trait, life_cycle, closure, r#box, rc, refcell
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
}