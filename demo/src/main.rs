
mod demo;

/*
// 如无特殊情况，不建议直接使用函数名
use crate::demo::hello_rust;
use crate::demo::data_type::data_type_main;
use crate::demo::variables::variables_main;
use crate::demo::ctrl_stream::ctrl_stream_main;
use crate::demo::ret_value::ret_value_main;
use crate::demo::owner::var::owner_var_main;
use crate::demo::owner::fnc;
use crate::demo::borrow::borrow_main;

fn main() {
    hello_rust::hello_rust_main();
    data_type_main();
    variables_main();
    ctrl_stream_main();
    ret_value_main();
    owner_var_main();
    fnc::main();
    crate::demo::owner::ret_value::main();
    borrow_main();
}
 */

use crate::demo::hello_rust;
use crate::demo::data_type;
use crate::demo::variables;
use crate::demo::ctrl_stream;
use crate::demo::ret_value;
use crate::demo::owner::var;
use crate::demo::owner::fnc;
use crate::demo::owner;
use crate::demo::borrow;
use crate::demo::slice;
use crate::demo::r#struct;
use crate::demo::r#enum;
use crate::demo::r#match;
use crate::demo::vector;
use crate::demo::string;
use crate::demo::hashmap;
use crate::demo::error;
use crate::demo::generics;
use crate::demo::r#trait;
use crate::demo::life_cycle;

fn main() {
    hello_rust::main();
    data_type::main();
    variables::main();
    ctrl_stream::main();
    ret_value::main();
    var::main();
    fnc::main();
    owner::ret_value::main();
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
}
 