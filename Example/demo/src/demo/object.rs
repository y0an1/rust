/// 封装
#[allow(unused)]
mod demo1 {
    pub struct AveragedCollection {
        list: Vec<i32>,
        average: f64,
    }

    impl AveragedCollection {
        pub fn add(&mut self, value: i32) {
            self.list.push(value);
            self.update_average();
        }

        pub fn remove(&mut self) -> Option<i32> {
            let result = self.list.pop();
            match result {
                Some(value) => {
                    self.update_average();
                    Some(value)
                }
                None => None,
            }
        }

        pub fn average(&self) -> f64 {
            self.average
        }

        fn update_average(&mut self) {
            let total: i32 = self.list.iter().sum();
            self.average = total as f64 / self.list.len() as f64;
        }
    }

    pub fn main() {}
}

/// trait 对象
#[allow(unused)]
mod demo2 {
    // 定义一个 trait 对象
    trait Draw {
        fn draw(&self);
    }

    struct Screen {
        // 创建了一个字段，类型是 Vec<Box<dyn Draw>>，内部存放的是实现了 Draw trait 的类型
        pub components: Vec<Box<dyn Draw>>,
    }

    impl Screen {
        pub fn run(&self) {
            for ele in self.components.iter() {
                ele.draw();
            }
        }
    }

    struct Button {
        width: i32,
        height: i32,
        label: String,
    }

    impl Draw for Button {
        fn draw(&self) {
            // 绘制一个按钮
        }
    }

    struct SelectBox {
        width: u32,
        height: u32,
        options: Vec<String>,
    }

    impl Draw for SelectBox {
        fn draw(&self) {
            // 绘制一个选项框
        }
    }

    pub fn main() {
        let screen = Screen {
            components: vec![
                Box::new(SelectBox {
                    width: 75,
                    height: 10,
                    options: vec![
                        String::from("Yes"),
                        String::from("No"),
                        String::from("Maybe"),
                    ],
                }),
                Box::new(Button {
                    width: 50,
                    height: 10,
                    label: String::from("OK"),
                }),
            ],
        };
        screen.run();
    }
}

/// 状态模式
#[allow(unused)]
mod demo3 {
    trait State {
        fn request_review(self: Box<Self>) -> Box<dyn State>;
        fn approve(self: Box<Self>) -> Box<dyn State>;
        fn content<'a>(&self, post: &'a Post) -> &'a str {
            ""
        }
    }

    struct Published {}
    impl State for Published {
        fn request_review(self: Box<Self>) -> Box<dyn State> {
            self
        }
        fn approve(self: Box<Self>) -> Box<dyn State> {
            self
        }
        fn content<'a>(&self, post: &'a Post) -> &'a str {
            &post.content
        }
    }

    struct PendingReview {}
    impl State for PendingReview {
        fn request_review(self: Box<Self>) -> Box<dyn State> {
            self
        }
        fn approve(self: Box<Self>) -> Box<dyn State> {
            Box::new(Published {})
        }
    }

    struct Draft {}
    impl State for Draft {
        fn request_review(self: Box<Self>) -> Box<dyn State> {
            Box::new(PendingReview {})
        }

        fn approve(self: Box<Self>) -> Box<dyn State> {
            self
        }
    }

    struct Post {
        state: Option<Box<dyn State>>,
        content: String,
    }
    impl Post {
        fn new() -> Post {
            Post {
                state: Some(Box::new(Draft {})),
                content: String::new(),
            }
        }

        fn add_text(&mut self, text: &str) {
            self.content.push_str(text);
        }

        fn content(&self) -> &str {
            self.state.as_ref().unwrap().content(&self)
        }

        fn request_review(&mut self) {
            if let Some(s) = self.state.take() {
                self.state = Some(s.request_review())
            }
        }

        fn approve(&mut self) {
            if let Some(s) = self.state.take() {
                self.state = Some(s.approve())
            }
        }
    }

    pub fn main() {
        let mut post = Post::new();
        post.add_text("I ate a salad for lunch today");
        assert_eq!("", post.content());

        post.request_review();
        assert_eq!("", post.content());

        post.approve();
        assert_eq!("I ate a salad for lunch today", post.content());
    }
}

/// 修改 demo3，将状态和行为编码为类型
#[allow(unused)]
mod demo4 {
    // 发布后的正式内容
    struct Post {
        content: String,
    }
    impl Post {
        fn new() -> DraftPost {
            DraftPost {
                content: String::new(),
            }
        }

        fn content(&self) -> &str {
            &self.content
        }
    }

    // 发布前的草稿
    struct DraftPost {
        content: String,
    }
    impl DraftPost {
        fn add_text(&mut self, text: &str) {
            self.content.push_str(text)
        }
        fn request_review(self) -> PendingReviewPost {
            PendingReviewPost {
                content: self.content,
            }
        }
    }

    // 待审批的内容
    struct PendingReviewPost {
        content: String,
    }
    impl PendingReviewPost {
        fn approve(self) -> Post {
            Post {
                content: self.content,
            }
        }
    }

    pub fn main(){
        // 这个 post 是发布前的草稿 post，所以它有添加文本的功能
        let mut post = Post::new();
        post.add_text("I ate a salad for lunch today");

        // 这个 post 是申请审批的 post，所以它有审核的功能
        let post = post.request_review();

        // 这个 post 就是正式内容的 post，所以它有查看内容的功能
        let post = post.approve();

        assert_eq!("I ate a salad for lunch today", post.content());
    }
}

pub fn main() {
    // demo1::main();
    // demo2::main();
    // demo3::main();
    // demo4::main();
}
