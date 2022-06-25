// 返回值同样会有所有权移动
fn _owner_ret_value() {
    let s1 = _gives_ownership();
    println!("s1: {}", s1);

    let s2 = String::from("hello");
    let s3 = _takes_and_gives_back(s2);
    // println!("s2: {}", s2); // borrow of moved value: `s2`
    println!("s3: {}", s3);
}

fn _gives_ownership() -> String {
    let some_string = String::from("hello");
    return some_string;
}

fn _takes_and_gives_back(a_string: String) -> String {
    return a_string;
}

pub fn main() {
    // _owner_ret_value();
}