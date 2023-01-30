/// 解引用原始指针
#[allow(unused)]
mod demo1 {
    pub fn main() {
        let mut num = 5;
        // 可以在不安全的代码块中创建原始指针，但是只能在不安全代码块中解引用
        let r1 = &num as *const i32;
        let r2 = &mut num as *mut i32;
        unsafe {
            println!("r1: {}", *r1); //r1: 5
            println!("r2: {}", *r2); //r2: 5
        }

        // 原始指针是不能保证其的有效性
        let addr = 0x0123456usize;
        let r = addr as *const i32;
        unsafe {
            println!("r: {}", *r);
        }
    }
}

/// unsafe 函数或方法
#[allow(unused)]
mod demo2 {
    unsafe fn dangerous() {}

    pub fn main() {
        unsafe {
            dangerous();
        }
    }
}

/// 创建 unsafe 代码的安全抽象
#[allow(unused)]
mod demo3 {
    // split_at_mut 的简易内部实现
    // fn split_at_mut(slice: &mut [i32], mid: usize) -> (&mut [i32], &mut [i32]) {
    //     let len = slice.len();
    //     assert!(mid <= len);
    //     (&mut slice[..mid], &mut slice[mid..])
    // }

    use core::slice;

    fn split_at_mut(slice: &mut [i32], mid: usize) -> (&mut [i32], &mut [i32]) {
        let len = slice.len();
        let ptr = slice.as_mut_ptr(); // 原始指针

        assert!(mid <= len);

        unsafe {
            (
                slice::from_raw_parts_mut(ptr, mid),
                slice::from_raw_parts_mut(ptr.add(mid), len - mid),
            )
        }
    }

    pub fn main() {
        let mut v = vec![1, 2, 3, 4, 5, 6];
        let r = &mut v[..];
        let (a, b) = r.split_at_mut(3);

        assert_eq!(a, &mut [1, 2, 3]);
        assert_eq!(b, &mut [4, 5, 6]);
    }
}

pub fn main() {
    // demo1::main();
    // demo2::main();
    // demo3::main();
}
