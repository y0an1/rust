// mod example::

// mod hello_rust;
// mod variables;
// mod data_type;
// mod control_stream;
// mod return_value;
// mod owner_var;
// mod owner_fnc;
// mod owner_ret_value;

mod demo;

use crate::demo::hello_rust;
use crate::demo::data_type::data_type_main;
use crate::demo::variables::variables_main;
use crate::demo::ctrl_stream::ctrl_stream_main;
use crate::demo::ret_value::ret_value_main;
use crate::demo::owner::var::owner_var_main;
use crate::demo::owner::fnc::owner_fnc_main;

fn main() {
    hello_rust::hello_rust_main();

    data_type_main();

    variables_main();

    ctrl_stream_main();

    ret_value_main();

    owner_var_main();

    owner_fnc_main();

    crate::demo::owner::ret_value::owner_ret_value_main();
}
 