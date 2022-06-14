const MAX_POINTS: i32 = 100_000;

fn main() {
    const MAX_POINTS: i32 = 100_000;

    // let x = 5;  // 不能对不可变的变量二次赋值
    let mut x = 5;  // 将该变量声明成可变的
    print!("The value of x is {}", x);

    x = 6; 
    print!("The value of x is {}", x);
}
