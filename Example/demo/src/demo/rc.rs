/// 两个 List 共享另一个 List 的所有权
#[allow(unused)]
mod demo1 {
    enum List {
        Cons(i32, Rc<List>),
        Nil,
    }

    use crate::demo::rc::demo1::List::{Cons, Nil};
    use std::rc::Rc;

    pub(super) fn main() {
        let a = Rc::new(Cons(5, Rc::new(Cons(10, Rc::new(Nil)))));
        println!("create a, strong count:{}, weak count:{}", Rc::strong_count(&a), Rc::weak_count(&a));

        // 每调用一次 clone， Rc 中的强引用计数就会加一
        let b = Cons(3, Rc::clone(&a));
        println!("create b, strong count:{}, weak count:{}", Rc::strong_count(&a), Rc::weak_count(&a));

        {
            let c = Cons(4, Rc::clone(&a));
            println!("create c, strong count:{}, weak count:{}", Rc::strong_count(&a), Rc::weak_count(&a));
        }
        println!("delete c, strong count:{}, weak count:{}", Rc::strong_count(&a), Rc::weak_count(&a));

    }
}

pub fn main() {
    // demo1::main();
}
