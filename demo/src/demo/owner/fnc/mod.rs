fn _take_owner_ship(some_string: String) {
    println!("{}", some_string);
}

fn _makes_copy(some_number: i32) {
    println!("{}", some_number)
}


fn _demo_1() {
    let s = String::from("Hello World");
    _take_owner_ship(s);
    // println!("s: {}", s);    // 此处内部调用了 move 功能
    // borrow of moved value: `s`
    // value borrowed here after move

    let x = 5;
    _makes_copy(x);
    println!("x: {}", x)    // 此处内部调用了 copy 功能
}

pub fn main() {
    // _demo_1();
}