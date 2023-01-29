
#[allow(unused)]
struct Rectangle {
    length: u32,
    width: u32,
}

impl Rectangle {

    #[allow(unused)]
    pub fn can_hold(&self, another: &Rectangle) -> bool {
        self.length > another.length && self.width > another.width
    }
}

#[allow(unused)]
fn add_two(a: i32) -> i32 {
    a + 2
}


#[allow(unused)]
struct Guess {
    value: u32,
}

impl Guess {

    // pub fn new (value: u32) -> Guess{
    //     if value < 1 || value > 100  {
    //         panic!("guess value must be between 1 and 100, get {}.", value)
    //     }
    //     else {
    //         Guess {
    //             value
    //         }
    //     }
    // }

    #[allow(unused)]
    pub fn new (value: u32) -> Guess{
        if value < 1  {
            panic!("guess value must be less than 1, get {}.", value)
        } else if value > 100 {
            panic!("guess value must be greater than 100, get {}.", value)
        } else {
            Guess {
                value
            }
        }
    }
}


#[cfg(test)]
#[allow(unused)]
mod tests {

    /// 使用外部的所有东西
    // use super::*;
    //
    // #[test]
    // fn larger_can_hold_smaller() {
    //     // 准备数据阶段
    //     let larger = Rectangle {
    //         width: 8,
    //         length: 10,
    //     };
    //     let smaller = Rectangle {
    //         width: 4,
    //         length: 5,
    //     };
    //
    //     // 断言
    //     assert!(larger.can_hold(&smaller));
    // }

    // #[test]
    // fn it_works() {
    //     assert_eq!(2+2, 4);
    // }

    /// 测试函数失败
    // #[test]
    // fn another() {
    //     // panic!("Make this test fail");
    // }


    /// assert_eq!() 和 assert_ne!() 的使用
    /// 测试私有函数
    // #[test]
    // fn it_works_add_two () {
    //     assert_eq!(4, add_two(3));
    //     // thread 'demo::test_module::tests::it_works_add_two' panicked at 'assertion failed: `(left == right)`
    //     //     left: `4`,
    //     //     right: `5`', src/demo/test_module/mod.rs:55:9
    //     // note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
    //     assert_ne!(4, add_two(2));
    // }

    /// 测试 should_panic 属性
    // #[test]
    // #[should_panic]
    // fn guess() {
    //     Guess::new(50);
    // }

    /// 更加详细的 should_panic
    // #[test]
    // #[should_panic(expected = "guess value must be greater than 100")]
    // fn great_than_100() {
    //     // Guess::new(200);
    //     Guess::new(0);
    // }

    /// 在测试中使用 Result<T, E>
    #[test]
    fn it_works() -> Result<(),String> {
        if 2+2 == 4 {
            Ok(())
        }else {
            Err(String::from("two add two does not equal four"))
        }
    }

    fn test_attr() {
        panic!("test arrt");
    }
}