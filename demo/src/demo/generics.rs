
#[allow(unused)]
mod demo1 {
    /// 泛型函数定义, 缺乏 trait 相关知识，目前仅知道如何定义即可
    pub fn main() {
        // let num_list = [1,2,3,4,5,6];
        // let rslt = largest(&num_list);
        // println!("The largest number is {}", rslt);
    }

    // fn largest<T:  std::cmp::PartialOrd>(list: &[T]) -> T {
    //     let mut largest = &list[0];
    //     for &item in list {
    //         if item > largest {     // 不是所有类型都可以进行比较，必须实现了 std::cmp::PartialOrd 这个 trait 才可以
    //             largest = item;
    //         }
    //     }
    //
    //     largest
    // }
}

#[allow(unused)]
mod demo2 {
    /// Struct 定义中的泛型
    #[derive(Debug)]
    struct Point<T> {
        x: T,
        y: T,
    }

    /// 方法定义中泛型
    impl<T> Point<T> {
        fn x(&self) -> &T {
            &self.x
        }
    }

    /// 指明类型的方法
    impl Point<i32> {
        /// 在i32的方法列表中，是看不到 x() 方法的，只有 x1()
        fn x1(&self) -> &i32 {
            &self.x
        }
    }

    #[derive(Debug)]
    pub struct Point2<T, U> {
        pub x: T,
        pub y: U,
    }

    /// 方法泛型类型参数与 struct 泛型类型不一样
    impl<T, U> Point2<T, U> {
        pub fn x(&self) ->&T {
            &self.x
        }

        pub fn y(&self) -> &U {
            &self.y
        }

        pub fn new(x:T, y:U) -> Point2<T,U> {
            Point2 {
                x,
                y,
            }
        }

        pub fn mixup<V, W>(self, other: Point2<V, W>) -> Point2<T, W> {
            Point2 {
                x: self.x,
                y: other.y,
            }
        }
    }

    pub fn main() {
        let integer = Point { x: 3, y: 4 };
        let float = Point { x: 3.5, y: 4.5 };
        println!("{:?}", integer);
        println!("{:?}", float);

        let point = Point2 { x: 3, y: 4.5 };
        println!("{:?}", point);
    }
}

/// enum 定义中的泛型
// enum Option<T> {
//     Some(T),
//     None,
// }
//
// enum Result<T, E> {
//     Ok(T),
//     Err(E),
// }

#[allow(unused)]
mod demo3 {
    use super::demo2::Point2;

    pub fn main() {
        let p1 = Point2 { x: 3, y: 4 };
        let p2 = Point2 { x: "hello", y: 'c' };
        let p3 = Point2::new(5,6);
        println!("p3.x: {}, p3.y: {}", p3.x, p3.y);

        let p3 = p1.mixup(p2);
        println!("p3.x: {}, p3.y: {}", p3.x(), p3.y());
    }

}

pub fn main() {
    // demo1::main();
    // demo2::main();
    // demo3::main();
}