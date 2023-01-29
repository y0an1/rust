/// 定义一个 trait
#[allow(unused)]
mod demo1 {
    trait Summary {
        /// 仅定义个 trait，没有实现
        fn summarize_author(&self) -> String;
    }

    /// 在类型上实现 trait
    struct NewArticle {
        pub headline: String,
        pub localtion: String,
        pub author: String,
        pub content: String,
    }

    impl Summary for NewArticle {
        // 具体实现
        fn summarize_author(&self) -> String {
            format!("@{}", self.author)
        }
    }

    struct Tweet {
        pub username: String,
        pub content: String,
        pub reply: bool,
        pub retweet: bool,
    }

    impl Summary for Tweet {
        fn summarize_author(&self) -> String {
            format!("@{}", self.retweet)
        }
    }

    pub fn main() {
        let tweet = Tweet {
            username: String::from("horse_ebooks"),
            content: String::from("of course, as you probably already know, people"),
            reply: false,
            retweet: false,
        };
        println!("1 new tweet: {}", tweet.summarize_author());
        //1 new tweet: horse_ebooks: of course, as you probably already know, people
    }
}

/// 使用 trait 的默认实现
#[allow(unused)]
mod demo2 {
    trait Summary {
        fn summarize_author(&self) -> String;
        /// 定义个 trait，并默认实现这个 trait
        fn summarize(&self) -> String {
            // format!("(Read more...)")

            // 调用没有默认实现的 trait 方法
            format!("(Read more from {}...)", self.summarize_author())
        }
    }

    /// 在类型上实现 trait
    struct NewArticle {
        pub headline: String,
        pub localtion: String,
        pub author: String,
        pub content: String,
    }

    impl Summary for NewArticle {
        fn summarize_author(&self) -> String {
            format!("@{}", self.author)
        }
    }

    struct Tweet {
        pub username: String,
        pub content: String,
        pub reply: bool,
        pub retweet: bool,
    }

    impl Summary for Tweet {
        fn summarize_author(&self) -> String {
            format!("@{}", self.retweet)
        }

        // 重写默认实现
        fn summarize(&self) -> String {
            format!("{}: {}", self.username, self.content)
        }
    }

    pub fn main() {
        let article = NewArticle {
            headline: String::from("Pengguins win the Stanley Cup Championship!"),
            content: String::from(
                "The Pittsburgh Penguins once again are the best hockey team in the NHL.",
            ),
            author: String::from("Iceburgh"),
            localtion: String::from("Pittsburgh, PA, USA"),
        };
        println!("1 new article: {}", article.summarize()); // 使用了默认实现-> 1 new article: (Read more...)
    }
}

/// trait 作为参数
#[allow(unused)]
mod demo3 {
    use std::fmt;

    trait Summary {
        fn summarize_author(&self) -> String;

        fn summarize(&self) -> String {
            // 调用没有默认实现的 trait 方法
            format!("(Read more from {}...)", self.summarize_author())
        }
    }

    struct NewArticle {
        pub headline: String,
        pub localtion: String,
        pub author: String,
        pub content: String,
    }

    impl Summary for NewArticle {
        fn summarize_author(&self) -> String {
            format!("@{}", self.author)
        }
    }

    struct Tweet {
        pub username: String,
        pub content: String,
        pub reply: bool,
        pub retweet: bool,
    }

    impl Summary for Tweet {
        fn summarize_author(&self) -> String {
            format!("@{}", self.retweet)
        }

        // 改写默认实现
        fn summarize(&self) -> String {
            format!("{}: {}", self.username, self.content)
        }
    }

    #[allow(unused)]
    /// impl trait，trait bound 语法糖
    fn notify(item: impl Summary) {
        println!("Breaking news! {}", item.summarize());
    }

    #[allow(unused)]
    /// trait bound 语法
    fn notify_trait_bound<T: Summary>(item: T) -> T {
        println!("Breaking news! {}", item.summarize());
        item
    }

    #[allow(unused)]
    /// 使用 + 来指定多个 trait bound
    fn notify_more_trait_bound<T: Summary + fmt::Display>(item: T) {
        println!("Breaking news! {}", item.summarize());
    }

    #[allow(unused)]
    /// 使用 where 子语句来指定多个 trait bound
    fn notify_more_trait_bound_use_where<T, U>(a: T, b: U) -> String
    where
        T: Summary + fmt::Display,
        U: Clone + fmt::Debug,
    {
        format!("Breaking news! {}", a.summarize())
    }

    #[allow(unused)]
    /// 使用 trait 作为返回值类型
    fn notify_ret(s: &str) -> impl Summary {
        NewArticle {
            headline: String::from("Pengguins win the Stanley Cup Championship!"),
            content: String::from(
                "The Pittsburgh Penguins once again are the best hockey team in the NHL.",
            ),
            author: String::from("Iceburgh"),
            localtion: String::from("Pittsburgh, PA, USA"),
        }
    }

    pub fn main() {
        let article = NewArticle {
            headline: String::from("Pengguins win the Stanley Cup Championship!"),
            content: String::from(
                "The Pittsburgh Penguins once again are the best hockey team in the NHL.",
            ),
            author: String::from("Iceburgh"),
            localtion: String::from("Pittsburgh, PA, USA"),
        };
        notify(article);

        let mut article = NewArticle {
            headline: String::from("Pengguins win the Stanley Cup Championship!"),
            content: String::from(
                "The Pittsburgh Penguins once again are the best hockey team in the NHL.",
            ),
            author: String::from("Iceburgh"),
            localtion: String::from("Pittsburgh, PA, USA"),
        };
        article = notify_trait_bound(article);
        
        // notify_more_trait_bound(article); // NewArticle 没有实现 Display 这个 trait，所以会报错
    }
}

/// 使用 Trait bound 修复错误
#[allow(unused)]
mod demo4 {

    fn largest<T: PartialOrd>(list: &[T]) -> &T {
        // 对 泛型进行约束，要求实现 PartialOrd 这个 trait
        let mut largest = &list[0];
        for item in list {
            // 不是所有类型都可以进行比较，必须实现了 std::cmp::PartialOrd 这个 trait 才可以
            // 另外在调试时发现，编译器会自动为 item 和 largest 进行解引用后再进行比较
            if item > largest {
                largest = item;
            }
        }

        largest
    }

    pub fn main() {
        let num_list = [1, 2, 33, 4, 5, 6];
        let rslt = largest(&num_list);
        println!("The largest number is {}", rslt); // 此处 rslt 是一个引用类型，但是 Rust 编译器在这里自动的进行解引用了

        let str_list = vec![String::from("hello"), String::from("world")];
        let rslt = largest(&str_list);
        println!("The largest str is {}", rslt);
    }
}

/// 使用 Trait bound 有条件的实现方法
#[allow(unused)]
mod demo5 {
    use std::fmt::Display;

    struct Pair<T> {
        x: T,
        y: T,
    }

    impl<T> Pair<T> {
        #[allow(unused)]
        /// 不论 T 是什么类型，都有一个 new 方法
        fn new(x: T, y: T) -> Self {
            Self { x, y }
        }
    }

    impl<T: Display + PartialOrd> Pair<T> {
        #[allow(unused)]
        /// 只要泛型类型 T 实现了 Display 和 PartialOrd，才会有 cpm_display 方法
        fn cpm_display(&self) {
            if self.x >= self.y {
                println!("The largest member is x: {}", self.x);
            } else {
                println!("The largest member is y: {}", self.y);
            }
        }
    }

    pub fn main() {}
}

pub fn main() {
    // demo1::main();
    // demo2::main();
    // demo3::main();
    // demo4::main();
    // demo5::main();
}
