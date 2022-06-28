
/// 返回值同样会有所有权移动
#[allow(unused)]
fn demo() {
    let s1 = gives_ownership();
    println!("s1: {}", s1);

    let s2 = String::from("hello");
    let s3 = takes_and_gives_back(s2);
    // println!("s2: {}", s2); // borrow of moved value: `s2`
    println!("s3: {}", s3);
}

fn gives_ownership() -> String {
    let some_string = String::from("hello");
    return some_string;
}

fn takes_and_gives_back(a_string: String) -> String {
    return a_string;
}

pub fn main() {
    // demo();
}