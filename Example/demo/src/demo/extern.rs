/// extern 调用外部代码
#[allow(unused)]
mod demo1 {
    extern "C" {
        fn abs(input: i32) -> i32;
    }

    pub fn main() {
        unsafe {
            println!("Absolute value of -3 according to C: {}", abs(-3));
        }
    }
}

/// 定义 ABI 接口给其它语言调用
#[allow(unused)]
mod demo2 {
    #[no_mangle]
    pub extern "C" fn call_from_c() {
        println!("Just called a Rust function from C!");
    }

    pub fn main() {}
}

pub fn main() {}