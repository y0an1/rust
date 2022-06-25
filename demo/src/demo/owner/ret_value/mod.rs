pub fn owner_ret_value_main() {
    owner_ret_value();
}

fn owner_ret_value() {
    let s1 = gives_ownership();
    println!("s1: {}", s1);

    let s2 = String::from("hello");
    let s3 = takes_and_gives_back(s2);
}

fn gives_ownership() -> String {
    let some_string = String::from("hello");
    return some_string;
}

fn takes_and_gives_back(a_string: String) -> String {
    return a_string;
}
