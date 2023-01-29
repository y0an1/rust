

#[allow(unused)]
mod demo1 {
    /// 常量
    const MAX_POINTS: u32 = 100_000;

    /// 可变变量
    pub fn main() {
        // let x = 5;  // 不能对不可变的变量二次赋值
        let mut x = 5;  // 将该变量声明成可变的
        println!("The value of x is {}", x);

        x = 6;
        println!("The value of x is {}", x);

        const MAX_POINTS: u32 = 100_000;
        println!("const value is {}", MAX_POINTS);
    }
}

#[allow(unused)]
mod demo2 {
    /// 变量隐藏
    pub fn shadow1() {
        let x = 5;
        // x = x + 1;  // 此处会有问题，因为 x 已经是一个不可变的变量，后续进行的是赋值操作
        let x = x + 1;  // 此处没问题，因为新声明的变量 x 覆盖（隐藏）了旧的变量 x，从而可以编译通过
        println!("The value of x is {}", x);
    }
    pub fn shadow2() {
        let spaces = "    ";
        let spaces = spaces.len();
        println!("{}", spaces);
    }
}

pub fn main() {
    // demo1::main();
    // demo2::shadow1();
    // demo2::shadow2();
}