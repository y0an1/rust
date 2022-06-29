
use std::fmt;
use std::fmt::Display;

/// 定义一个 trait
#[allow(unused)]
fn demo1() {
    let tweet = Tweet {
        username: String::from("horse_ebooks"),
        content: String::from("of course, as you probably already know, people"),
        reply: false,
        retweet: false,
    };
    println!("1 new tweet: {}",tweet.summarize());
    //1 new tweet: horse_ebooks: of course, as you probably already know, people
}

#[allow(unused)]
pub trait Summary {
    /// 仅定义个 trait，没有实现
    // fn summarize(&self) -> String;

    fn summarize_author(&self) -> String;

    /// 定义个 trait，并默认实现这个 trait
    fn summarize(&self) -> String {
        // format!("(Read more...)")

        // 调用没有默认实现的 trait 方法
        format!("(Read more from {}...)", self.summarize_author())
    }
}

/// 在类型上实现 trait
#[allow(unused)]
pub struct NewArticle {
    pub headline: String,
    pub localtion: String,
    pub author: String,
    pub content: String,
}

#[allow(unused)]
impl Summary for NewArticle {
    // 具体实现
    // fn summarize(&self) -> String {
    //     format!("{}, by {} ({})", self.headline, self.author, self.localtion)
    // }

    fn summarize_author(&self) -> String {
        format!("@{}", self.author)
    }
}

#[allow(unused)]
pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}

#[allow(unused)]
impl Summary for Tweet {
    fn summarize_author(&self) -> String {
        format!("@{}", self.retweet)
    }

    // 具体实现
    fn summarize(&self) -> String {
        format!("{}: {}", self.username, self.content)
    }
}

/// 使用 trait 的默认实现
#[allow(unused)]
fn demo2() {
    let article = NewArticle{
        headline: String::from("Pengguins win the Stanley Cup Championship!"),
        content: String::from("The Pittsburgh Penguins once again are the best hockey team in the NHL."),
        author: String::from("Iceburgh"),
        localtion:String::from("Pittsburgh, PA, USA"),
    };
    println!("1 new article: {}", article.summarize()); // 使用了默认实现
    //1 new article: (Read more...)
}

/// trait 作为参数
#[allow(unused)]
fn demo3() {
    let article = NewArticle{
        headline: String::from("Pengguins win the Stanley Cup Championship!"),
        content: String::from("The Pittsburgh Penguins once again are the best hockey team in the NHL."),
        author: String::from("Iceburgh"),
        localtion:String::from("Pittsburgh, PA, USA"),
    };
    // notify(article);
    // notify_trait_bound(article);
    // notify_more_trait_bound(article);
}

#[allow(unused)]
fn notify(item: impl Summary) {
    println!("Breaking news! {}", item.summarize());
}

/// trait bound 语法
#[allow(unused)]
fn notify_trait_bound<T: Summary> (item: T) {
    println!("Breaking news! {}", item.summarize());
}

/// 使用 + 来指定多个 trait bound
#[allow(unused)]
fn notify_more_trait_bound<T: Summary + fmt::Display> (item: T) {
    println!("Breaking news! {}", item.summarize());
}

/// 使用 where 子语句来指定多个 trait bound
#[allow(unused)]
fn notify_more_trait_bound_use_where<T, U> (a:T, b:U) -> String
where
    T: Summary + fmt::Display,
    U: Clone + fmt::Debug,
{
    format!("Breaking news! {}", a.summarize())
}

/// 使用 trait 作为返回值类型
#[allow(unused)]
fn notify_ret(_s: &str) -> impl Summary {
    NewArticle{
        headline: String::from("Pengguins win the Stanley Cup Championship!"),
        content: String::from("The Pittsburgh Penguins once again are the best hockey team in the NHL."),
        author: String::from("Iceburgh"),
        localtion:String::from("Pittsburgh, PA, USA"),
    }
}

/// 使用 Trait bound 修复错误
#[allow(unused)]
fn demo4() {
    let num_list = [1,2,33,4,5,6];
    let rslt = largest(&num_list);
    println!("The largest number is {}", rslt); // 此处 rslt 是一个引用类型，但是 Rust 编译器在这里自动的进行解引用了

    let str_list = vec![String::from("hello"), String::from("world")];
    let rslt = largest(&str_list);
    println!("The largest str is {}", rslt);
}

fn largest<T:  PartialOrd>(list: &[T]) -> &T {
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

/// 使用 Trait bound 有条件的实现方法
#[allow(unused)]
struct Pair<T> {
    x: T,
    y: T,
}

impl<T> Pair<T> {
    /// 不论 T 是什么类型，都有一个 new 方法
    #[allow(unused)]
    fn new(x:T, y:T) -> Self{
        Self {x, y}
    }
}

impl<T: Display + PartialOrd> Pair<T> {
    /// 只要 泛型类型 T 实现了 Display 和 PartialOrd，才会有 cpm_display 方法
    #[allow(unused)]
    fn cpm_display(&self) {
        if self.x >= self.y {
            println!("The largest member is x: {}", self.x);
        }else {
            println!("The largest member is y: {}", self.y);
        }
    }
}

pub fn main() {
    // demo1();
    // demo2();
    // demo3();
    // demo4();
}