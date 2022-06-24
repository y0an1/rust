
mod hello_rust;
mod variables;
mod data_type;
mod control_stream;
mod function;


fn main() {
    hello_rust::println();

    data_type::base_type();
    data_type::tuple_type();
    data_type::array_type();

    variables::mutable();
    variables::shadow_1();
    variables::shadow_2();

    control_stream::ctrl_if();
    control_stream::ctrl_loop();
    control_stream::ctrl_while();
    control_stream::ctrl_for();
    control_stream::use_range();

    function::return_value();
    function::func_return_value();
}
 