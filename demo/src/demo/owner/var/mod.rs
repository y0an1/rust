pub fn owner_var_main() {
    func_move();
    func_clone();
}


// move 功能
fn func_move() {
    let s1 = String::from("hello");

    // s1 是一个指向堆内存的变量，s1 作为等号的右值，那
    // 么其值会根据 rust 的所有权规则，触发 move 现象，
    // 即：s1 所指向的堆内存被移动到 s2，s1 则被无效化
    let s2 = s1;

    // 当代码运行到此处时，s1 已经无效化了，所以此时 s1 变量是不可用的，代码运行到此处会报错
    // println!("s1 : {}", s1);
}   // 因为 s1 已经不可用所以不会触发 drop 函数，而 s2 指向了堆内存，所以会触发 drop 函数


// clone 功能
fn func_clone() {
    let s1 = String::from("hello");
    let s2 = s1.clone();

    println!("{} {}", s1, s2);
}