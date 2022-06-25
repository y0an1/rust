
// 常量
const _MAX_POINTS: u32 = 100_000;

// 可变变量
fn _mutable() {
    // let x = 5;  // 不能对不可变的变量二次赋值
    let mut x = 5;  // 将该变量声明成可变的
    println!("The value of x is {}", x);

    x = 6;
    println!("The value of x is {}", x);


    const MAX_POINTS: u32 = 100_000;
    println!("const value is {}", MAX_POINTS);
}

// 变量隐藏
fn _shadow_one() {
    let x = 5;
    // x = x + 1;  // 此处会有问题，因为 x 已经是一个不可变的变量，后续进行的是赋值操作
    let x = x + 1;  // 此处没问题，因为新声明的变量 x 覆盖（隐藏）了旧的变量 x，从而可以编译通过
    println!("The value of x is {}", x);
}

fn _shadow_two() {
    let spaces = "    ";
    let spaces = spaces.len();
    println!("{}", spaces);
}

pub fn main() {
    // _mutable();
    // _shadow_one();
    // _shadow_two();
}