/// 建立 module
#[allow(unused)]
mod front_of_house {
    mod hosting {
        fn add_to_waitlist() {}

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
use crate::demo::{hello_rust, data_type, variables, ctrl_stream, ret_value, owner, borrow, slice, r#struct, r#enum, r#match, vector, string, hashmap, error, generics, r#trait, life_cycle, closure};

pub fn entry_fnc() {
    hello_rust::main();
    data_type::main();
    variables::main();
    ctrl_stream::main();
    ret_value::main();
    owner::main();
    borrow::main();
    slice::main();
    r#struct::main();
    r#enum::main();
    r#match::main();
    vector::main();
    string::main();
    hashmap::main();
    error::main();
    generics::main();
    r#trait::main();
    life_cycle::main();
    closure::main();
}