
/// 在这个例子中，为了不丢失 s1 的数据，需要定义 s2 来接收 s1 的所有权
#[allow(unused)]
fn demo1() {
    let s1 = String::from("hello world");
    let (s2, len) = calc_length(s1);

    println!("s2: {}, len: {}", s2, len);
}

fn calc_length(s: String) -> (String, usize) {
    let len = s.len();
    return (s, len);
}

/// 在同样的例子中， rust 为了确保仅处理数据，而不必处理多个变量，提出了“引用”特性
///
/// 当一个引用作为函数参数时，这个行为叫做借用
///
/// 默认借用的东西是不能更改的
#[allow(unused)]
fn demo2() {
    let s1 = String::from("hello world");
    let len = calc_len_ref(&s1); // 此处传递的是 s1 的引用，并没有涉及所有权

    println!("s1: {}, len: {}", s1, len);
}

fn calc_len_ref(s: &String) -> usize {
    return s.len();
}

/// 如果要修改借用的东西，则需要将其声明为可变
#[allow(unused)]
fn demo3() {
    let mut s1 = String::from("hello world");
    let len = calc_len_mut_ref(&mut s1);

    println!("s1: {}, len: {}", s1, len);
}

fn calc_len_mut_ref(s: &mut String) -> usize {
    s.push_str(" rust");
    return s.len();
}

/// 在同一个作用域内，有且仅有一个可变引用
#[allow(unused)]
fn demo4() {
    let mut s = String::from("hello");
    let s1 = &mut s;
    // let s2 = &mut s;

    println!("s1: {}", s1);
    // println!("s2: {}", s2);

    /*
    error[E0499]: cannot borrow `s` as mutable more than once at a time
    --> src/demo/borrow/struct:45:14
    |
    44 |     let s1 = &mut s;
    |              ------ first mutable borrow occurs here
    45 |     let s2 = &mut s;
    |              ^^^^^^ second mutable borrow occurs here
    46 |
    47 |     println!("s2: {}, s3: {}", s1, s2);
    |                                -- first borrow later used here
    */
}

/// 创建多个作用域来允许多个可变引用
#[allow(unused)]
fn demo5() {
    let mut s = String::from("hello");

    {
        let s1 = &mut s;
        println!("s1: {}", s1);
    }

    let s2 = &mut s;
    println!("s2: {}", &s2);
}

/// 一个变量不可以同时有不可变引用和可变引用
#[allow(unused)]
fn demo6() {
    let mut s = String::from("hello");
    let r1 = &s;
    let r2 = &s;
    println!("r1:{}, r2:{}", r1, r2);

    // let s1 = &mut s;
    /*
        error[E0502]: cannot borrow `s` as mutable because it is also borrowed as immutable
        --> src/demo/borrow/struct:81:14
        |
        79 |     let r1 = &s;
        |              -- immutable borrow occurs here
        80 |     let r2 = &s;
        81 |     let s1 = &mut s;
        |              ^^^^^^ mutable borrow occurs here
        82 |
        83 |     println!("r1:{}, r2:{}, s1:{}", r1,r2,s1);
        |                                     -- immutable borrow later used here
    */

    // println!("s1:{}", s1);
}

/// 悬空引用
#[allow(unused)]
// fn demo7() {
//     let r = dangle();
// }
//
// fn dangle() -> &String{
//     let s = String::from("hello");
//     return &s;
//
//     // error[E0106]: missing lifetime specifier
//     //    --> src/demo/borrow/struct:106:16
//     //     |
//     // 106 | fn dangle() -> &String{
//     //     |                ^ expected named lifetime parameter
//     //     |
//     //     = help: this function's return type contains a borrowed value, but there is no value for it to be borrowed from
//     // help: consider using the `'static` lifetime
//     //     |
//     // 106 | fn dangle() -> &'static String{
//     //     |                ~~~~~~~~
// }


pub fn main() {
    // demo1();
    // demo2();
    // demo3();
    // demo4();
    // demo5();
    // demo6();
    // demo7();
}
