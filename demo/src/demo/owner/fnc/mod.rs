
#[allow(unused)]
fn demo1() {
    let s = String::from("Hello World");
    take_owner_ship(s);
    // println!("s: {}", s);    // 此处内部调用了 move 功能
    // borrow of moved value: `s`
    // value borrowed here after move

    let x = 5;
    makes_copy(x);
    println!("x: {}", x)    // 此处内部调用了 copy 功能
}

fn take_owner_ship(some_string: String) {
    println!("{}", some_string);
}

fn makes_copy(some_number: i32) {
    println!("{}", some_number)
}

pub fn main() {
    // demo1();
}