/// 创建新线程
#[allow(unused)]
mod demo1 {
    use std::{thread, time::Duration};

    pub fn main() {
        thread::spawn(|| {
            for i in 1..10 {
                println!("hi number {} from the spawned thread!", i);
                thread::sleep(Duration::from_millis(1));
            }
        });

        for i in 1..5 {
            println!("hi number {} from the main thread!", i);
            thread::sleep(Duration::from_millis(1));
        }
    }
}

/// 等待线程完成
#[allow(unused)]
mod demo2 {
    use std::{thread, time::Duration};

    pub fn main() {
        let handle = thread::spawn(|| {
            for i in 1..10 {
                println!("hi number {} from the spawned thread!", i);
                thread::sleep(Duration::from_millis(1));
            }
        });

        for i in 1..5 {
            println!("hi number {} from the main thread!", i);
            thread::sleep(Duration::from_millis(1));
        }

        handle.join().unwrap();
    }
}

/// move 闭包
#[allow(unused)]
mod demo3 {
    use std::{thread, time::Duration};

    pub fn main() {
        let v = vec![1, 2, 3];
        // let handle = thread::spawn( || {
        let handle = thread::spawn(move || {
            // closure may outlive the current function, but it borrows `v`, which is owned by the current function
            println!("Here's a vector: {:?}", v);
        });

        handle.join().unwrap();
    }
}

/// channel 发送消息
#[allow(unused)]
mod demo4 {
    use std::{sync::mpsc, thread, time::Duration};

    pub fn main() {
        let (tx, rx) = mpsc::channel();

        let handle = thread::spawn(move || {
            let val = String::from("hi");
            tx.send(val).unwrap();
        });

        // recv 会阻塞线程，直到有数据来
        // let received = rx.recv().unwrap();
        // println!("Got: {}", received);

        // 两个代码是一样的，只是 try_recv 不会阻塞线程
        loop {
            let received = rx.try_recv();

            match received {
                Ok(msg) => {
                    println!("Got: {}", msg);
                    break;
                }
                Err(error) => {
                    if let mpsc::TryRecvError::Empty = error {
                        // println!("msg is empty");
                    } else {
                        panic!("发生错误： {}", error);
                    }
                }
            }
        }
    }
}

/// channel 发送多消息
#[allow(unused)]
mod demo5 {
    use std::{sync::mpsc, thread, time::Duration};

    pub fn main() {
        let (tx, rx) = mpsc::channel();

        let handle = thread::spawn(move || {
            let vals = vec![
                String::from("msg"),
                String::from("from"),
                String::from("the"),
                String::from("thread"),
            ];

            for ele in vals {
                tx.send(ele).unwrap();
                thread::sleep(Duration::from_millis(500));
            }
        });

        // 相当于隐式调用 recv 函数
        for ele in rx {
            println!("Got: {}", ele);
        }
    }
}

/// 克隆发送端
#[allow(unused)]
mod demo6 {
    use std::{sync::mpsc, thread, time::Duration};

    pub fn main() {
        let (tx, rx) = mpsc::channel();

        let tx1 = mpsc::Sender::clone(&tx);
        thread::spawn(move || {
            let vals = vec![
                String::from("1: msg"),
                String::from("1: from"),
                String::from("1: the"),
                String::from("1: thread"),
            ];

            for ele in vals {
                tx1.send(ele).unwrap();
                thread::sleep(Duration::from_millis(500));
            }
        });

        thread::spawn(move || {
            let vals = vec![
                String::from("msg"),
                String::from("from"),
                String::from("the"),
                String::from("thread"),
            ];

            for ele in vals {
                tx.send(ele).unwrap();
                thread::sleep(Duration::from_millis(500));
            }
        });

        // 多发送端时，顺序无法保证
        for ele in rx {
            println!("Got: {}", ele);
        }
    }
}

/// Mutex 使用
#[allow(unused)]
mod demo7 {
    use std::sync::Mutex;

    pub fn main() {
        let m = Mutex::new(5);
        {
            // lock 有可能会失败，所以要进行判断
            let mut num = m.lock().unwrap();
            *num = 6;
        }
        println!("m = {:?}", m);
    }
}

/// 多线程共享 Mutex
#[allow(unused)]
mod demo8 {
    use std::{
        sync::{Arc, Mutex},
        thread,
    };

    pub fn main() {
        let counter = Arc::new(Mutex::new(0));
        let mut handles = vec![];

        for _ in 0..10 {
            let counter = Arc::clone(&counter);
            let handle = thread::spawn(move || {
                let mut num = counter.lock().unwrap();
                *num += 1;
            });
            handles.push(handle);
        }

        for ele in handles {
            ele.join().unwrap();
        }

        println!("counter = {}", *counter.lock().unwrap());
    }
}

pub fn main() {
    // demo1::main();
    // demo2::main();
    // demo3::main();
    // demo4::main();
    // demo5::main();
    // demo6::main();
    // demo7::main();
    // demo8::main();
}
