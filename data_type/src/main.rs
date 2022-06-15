fn main() {
    // error[E0282]: type annotations needed
    // 在编译器推导中，42解析成的数字有u32，i32等多个数据类型可以标注
    // 所以必须手动指定该变量的类型
    // let guess = "42".parse().expect("Not a number");
    let guess: u32 = "42".parse().expect("Not a number");
    println!("guess value: {}", guess);
}
