
#[allow(unused)]
mod demo1 {
    /// 所有权函数参数
    pub fn main() {
        let s = String::from("Hello World");
        take_owner_ship(s);
        // println!("s: {}", s);    // 此处内部调用了 move 功能
        // borrow of moved value: `s`
        // value borrowed here after move

        let x = 5;
        makes_copy(x);
        println!("x: {}", x)    // 此处内部调用了 copy 功能
    }

    fn take_owner_ship(some_string: String) {
        println!("{}", some_string);
    }

    fn makes_copy(some_number: i32) {
        println!("{}", some_number)
    }
}

#[allow(unused)]
mod demo2 {
    /// 返回值所有权
    pub fn main() {
        let s1 = gives_ownership();
        println!("s1: {}", s1);

        let s2 = String::from("hello");
        let s3 = takes_and_gives_back(s2);
        // println!("s2: {}", s2); // borrow of moved value: `s2`
        println!("s3: {}", s3);
    }

    fn gives_ownership() -> String {
        let some_string = String::from("hello");
        return some_string;
    }

    fn takes_and_gives_back(a_string: String) -> String {
        return a_string;
    }
}

#[allow(unused)]
mod demo3 {
    /// 变量所有权
    /// move 功能
    pub fn move_fnc() {
        let s1 = String::from("hello");

        // s1 是一个指向堆内存的变量，s1 作为等号的右值，那
        // 么其值会根据 rust 的所有权规则，触发 move 现象，
        // 即：s1 所指向的堆内存被移动到 s2，s1 则被无效化
        let _s2 = s1;

        // 当代码运行到此处时，s1 已经无效化了，所以此时 s1 变量是不可用的，代码运行到此处会报错
        // println!("s1 : {}", s1);
    }   // 因为 s1 已经不可用所以不会触发 drop 函数，而 s2 指向了堆内存，所以会触发 drop 函数


    /// clone 功能
    pub fn clone_fnc() {
        let s1 = String::from("hello");
        let s2 = s1.clone();

        println!("{} {}", s1, s2);
    }
}

pub fn main() {
    // demo1::main();
    // demo2::main();
    // demo3::move_fnc();
    // demo3::clone_fnc();
}