/// Rc<T> 和 RefCell<T> 循环引用导致内存泄漏
#[allow(unused)]
mod demo1 {
    use self::List::{Cons, Nil};
    use std::{cell::RefCell, collections::btree_map::IterMut, rc::Rc};

    #[derive(Debug)]
    enum List {
        Cons(i32, RefCell<Rc<List>>),
        Nil,
    }

    impl List {
        fn tail(&self) -> Option<&RefCell<Rc<List>>> {
            match self {
                Cons(_, item) => Some(item),
                Nil => None,
            }
        }
    }

    pub fn main() {
        let a = Rc::new(Cons(5, RefCell::new(Rc::new(Nil))));
        println!("a initial rc count = {}", Rc::strong_count(&a));
        println!("a next item = {:?}", a.tail());

        let b = Rc::new(Cons(10, RefCell::new(Rc::clone(&a))));
        println!("a rc count after b creation = {}", Rc::strong_count(&a));
        println!("b initial rc count = {}", Rc::strong_count(&b));
        println!("a next item = {:?}", b.tail());

        if let Some(link) = a.tail() {
            *link.borrow_mut() = Rc::clone(&b);
        }

        println!("b rc count after changing a = {}", Rc::strong_count(&b));
        println!("a rc count after changing a = {}", Rc::strong_count(&a));

        // thread 'main' has overflowed its stack
        // fatal runtime error: stack overflow
        println!("a nex item = {:?}", a.tail()); // 栈溢出了
    }
}

/// 弱引用解决循环引用问题
#[allow(unused)]
mod demo2 {
    use std::{
        borrow::{Borrow, BorrowMut},
        cell::RefCell,
        rc::{Rc, Weak},
    };

    #[derive(Debug)]
    struct Node {
        value: i32,
        parent: RefCell<Weak<Node>>,      // 用来找到树枝
        children: RefCell<Vec<Rc<Node>>>, // 用来找到叶子
    }

    pub fn main() {
        // 创建树，叶子可以找到对应的树枝，树枝可以找到所有的叶子

        // 叶子
        let leaf = Rc::new(Node {
            value: 3,
            parent: RefCell::new(Weak::new()),
            children: RefCell::new(vec![]),
        });

        // 打印出当前叶子所在的树枝
        println!("leaf parent = {:?}", leaf.parent.borrow().upgrade());

        println!(
            "leaf strong ={}, weak = {}",
            Rc::strong_count(&leaf),
            Rc::weak_count(&leaf)
        );

        {
            // 树枝
            let branch = Rc::new(Node {
                value: 5,
                parent: RefCell::new(Weak::new()),
                children: RefCell::new(vec![Rc::clone(&leaf)]),
            });

            // 树枝绑定叶子
            *leaf.parent.borrow_mut() = Rc::downgrade(&branch);

            // 打印叶子所在的树枝
            println!("leaf parent = {:?}", leaf.parent.borrow().upgrade());

            println!(
                "branch strong ={}, weak = {}",
                Rc::strong_count(&branch),
                Rc::weak_count(&branch)
            );

            println!(
                "leaf strong ={}, weak = {}",
                Rc::strong_count(&leaf),
                Rc::weak_count(&leaf)
            );
        }

        // 打印叶子所在的树枝
        println!("leaf parent = {:?}", leaf.parent.borrow().upgrade());

        println!(
            "leaf strong ={}, weak = {}",
            Rc::strong_count(&leaf),
            Rc::weak_count(&leaf)
        );
    }
}

pub fn main() {
    // demo1::main();
    // demo2::main();
}
