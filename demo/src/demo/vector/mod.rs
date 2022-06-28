
/// 创建 vector
#[allow(unused)]
fn demo1() {
    // 使用标准的 new 函数进行创建
    let mut v = Vec::new();
    v.push(1);
    v.push(2);
    v.push(3);

    // 使用宏+初始值进行创建
    let mut v = vec![1,2,3,];
}

/// 读取 vector
#[allow(unused)]
fn demo2() {
    let mut v = vec![1,2,3,4,5,6];
    let third = &v[2];
    println!("The third element is {}" ,third);

    match v.get(2) {
        Some(third) => println!("The third element is {}" ,third),
        None => println!("There is no third element"),
    }
}

/// 所有权和借用规则
#[allow(unused)]
fn demo3() {
    let mut v = vec![1,2,3,4,5,6];
    let first = &v[0];                          // 此处是一个不可变的引用
    // v.push(7);                                  // 此处是一个可变引用
    println!("The first element is {}", first);       // 此处是一个不可变的引用
    // 根据所有权的借用规则，同一个作用域内不能同时存在可变和不可变的引用，编译器抛出错误
    // 出现此问题的情况：当执行第 33 行时，有可能内存中没有那么大的空间，需要重新申请，从而导致 32 行变成一个悬空引用
}

/// 遍历 vector
#[allow(unused)]
fn demo4() {
    let v = vec![1, 2, 3, 4];
    for i in &v {
        println!("{}", i);
    }

    for (i, item) in v.iter().enumerate() {
        println!("enumerate: i: {}, item: {}", i, item);
    }
}

/// 更改 vector 的值
#[allow(unused)]
fn demo5() {
    let mut v = vec![1, 2, 3, 4];
    for i in &mut v {
        *i += 10;
    }

    for x in &v {
        println!("x: {}", x);
    }
}

/// 存放多多种数据类型
#[derive(Debug)]
#[allow(unused)]
enum MultTypeEnum {
    Int(i32),
    Float(f64),
    Text(String),
}

#[allow(unused)]
fn demo6() {
    let v = vec![
        MultTypeEnum::Float(3.14),
        MultTypeEnum::Int(8),
        MultTypeEnum::Text(String::from("hello")),
    ];

    println!("{:?}", v);    // [Float(3.14), Int(8), Text("hello")]
}

pub fn main() {
    // demo1();
    // demo2();
    // demo3();
    // demo4();
    // demo5();
    // demo6();
}