pub fn return_value() {
    let x = 5;
    let y = {
        let x = 1;
        x + 3   // 此处是一个表达式，在 rust 中，最后一行是一个表达式的话，其值将会被返回
        // x + 3; // 此处是一个语句，语句是没有返回值，所以此处的 y 是一个 “()”，空的 tuple
    };

    println!("The value of x is: {}", x); //The value of x is: 5
    println!("The value of y is: {}", y); //The value of y is: 4
}


// 函数的返回值
pub fn func_return_value() {
    let x = five();

    //The value of x is: 5
    //The value of x is: 6
    println!("The value of x is: {}", x);
}

fn five() -> i32{
    // 5   // 函数最末尾的表达式的值默认为函数返回值
    return 6;   // 当使用 return 时，则以该关键字后面的表达式为返回值
}