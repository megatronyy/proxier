#![deny(rust_2018_idioms, clippy::disallowed_methods, clippy::disallowed_types)]
#![forbid(unsafe_code)]

use std::future::Future;
use std::pin::Pin;
use std::task::{Context, Poll};
use std::thread::sleep;
use std::time::{Duration, Instant};
// use tokio::sync::mpsc;
// pub use tracing::{debug, error, info, warn};

mod rt;

// proxy
// fn main () {
//     let (shutdown_tx, mut shutdow_rx) = mpsc::unbounded_channel();
//     let bind =BindT
//
//     tokio::select! {
//
//     }
// }

// 同步执行
// fn main() {
//     println!("Hello before reading file1");
//     let file1_contents = read_from_file1();
//     println!("Hello after reading file1");
//     let file2_contents = read_from_file2();
//     println!("{:?}", file2_contents);
//     println!("Hello after reading file2");
// }

// 多线程执行
// fn main() {
//     println!("Hello before reading file!");
//     let handle1 = std::thread::spawn(|| {
//         let file1_contents = read_from_file1();
//         println!("{:?}", file1_contents);
//     });
//
//     let handle2 = std::thread::spawn(|| {
//         let file2_contents = read_from_file2();
//         println!("{:?}", file2_contents);
//     });
//
//     handle1.join().unwrap();
//     handle2.join().unwrap();
// }

/*
1、异步运行, Rust异步的核心就是Future
2、谁来调用Future里的poll方法呢？是异步执行器，它是异步运行时的一部分。
  异步执行器会管理一个Future的集合，并通过调用Future上的poll方法来驱动他们完成。
  所以函数或者代码块在前面加上async关键字后，就相当于告诉异常执行器他会返回Future，这是Future需要被驱动直到完成。
3、但是异步执行器怎么知道异步已经准备好可以取得进度（可以产生值）了呢？他会持续不断的调用poll方法吗？
   a、Tokio执行器会一直不断的对pending状态的Future进行poll吗？肯定不会一直poll，
     Tokio(Rust的异步设计)是使用一个Waker组件来处理这件事的。

     当被异步执行器poll过的任务还没有准备好产生值的时候，这个任务就被注册到一个Waker.
     Waker会有一个处理程序（handle），它会被存储在任务关联的Context对象中。

     Waker有一个wake()方法，可以用来告诉异步执行器关联的任务应该被唤醒了。当wake()方法被调用了，
     Tokio执行器就会被通知是时候再次poll这个异常的任务了，具体方式就是调用任务上的poll()函数。

4、Tokio运行时就是管理异步任务并安排他们在CPU上执行的组件，一个程序可能生成多个任务，每个任务可能包含一个或多个Future
5、Tokio运行时，会注册异步的处理程序，以便在事件发生时，作为I/O操作的一部分进行调用。
  而在Tokio运行时里面，从内核监听这些事件并与Tokio其它部分通信的组件就是反应器（Reactor）
*/
// #[tokio::main]
// async fn main() {
//     println!("Hello before reading file!");
//
//     let h1 = tokio::spawn(async {
//         let _file1_contents = read_from_file1().await;
//     });
//
//     let h2 = tokio::spawn(async {
//         let _file2_contents = read_from_file2().await;
//     });
//     let _ = tokio::join!(h1, h2);
// }

// async fn read_from_file1() -> String {
//     sleep(Duration::new(4, 0));
//     println!("{:?}", "Processing file 1");
//     String::from("Hello, there from file 1")
// }
//
// async fn read_from_file2() -> String {
//     sleep(Duration::new(2, 0));
//     println!("{:?}", "Processing file 2");
//     String::from("Hello, there from file 2")
// }

// fn read_from_file1() -> impl Future<Output=String> {
//     async {
//         sleep(Duration::new(4, 0));
//         println!("{:?}", "Processing file 1");
//         String::from("Hello, there from file 1")
//     }
// }
//
// fn read_from_file2() -> impl Future<Output=String> {
//     async {
//         sleep(Duration::new(2, 0));
//         println!("{:?}", "Processing file 2");
//         String::from("Hello, there from file 2")
//     }
// }

// struct ReadFileFuture {}
//
// impl Future for ReadFileFuture {
//     type Output = String;
//
//     fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
//         println!("Tokio! Stop polling me");
//         // 告诉异步运行时(tokio)，让其再次poll这个Future
//         cx.waker().wake_by_ref();
//         // 此状态为不可用，当为此状态，tokio会把future注册到waker
//         // Poll::Pending
//         Poll::Ready(String::from("Hello, there from file 1"))
//     }
// }
//
// #[tokio::main]
// async fn main() {
//     println!("Hello before reading file!");
//
//     let h1 = tokio::spawn(async {
//         let future1 = ReadFileFuture {};
//         future1.await;
//     });
//
//     let h2 = tokio::spawn(async {
//         let file2_contents = read_from_file2().await;
//         print!("{:?}", file2_contents);
//     });
//
//     let _ = tokio::join!(h1, h2);
// }
//
// fn read_from_file2() -> impl Future<Output=String> {
//     async {
//         sleep(Duration::new(2, 0));
//         println!("{:?}", "Processing file 2");
//         String::from("Hello, there from file 2")
//     }
// }


struct AsyncTimer {
    expiration_time: Instant,
}

impl Future for AsyncTimer {
    type Output = String;

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        if Instant::now() >= self.expiration_time {
            println!("Hello, it's time for Future 1");
            Poll::Ready(String::from("Future 1 has completed"))
        } else {
            println!("Hello, it's not yet time for Future 1. Going to sleep");

            let waker = cx.waker().clone();
            let expriation_time = self.expiration_time;
            std::thread::spawn(move || {
                let current_time = Instant::now();
                if current_time < expriation_time {
                    std::thread::sleep(expriation_time - current_time);
                }
                waker.wake();
            });

            Poll::Pending
        }
    }
}

#[tokio::main]
async fn main() {
    let h1 = tokio::spawn(async {
        let future1 = AsyncTimer {
            expiration_time: Instant::now() + Duration::from_millis(4000),
        };
        println!("{:?}", future1.await);
    });

    let h2 = tokio::spawn(async {
        let file2_contents = read_from_file2().await;
        print!("{:?}", file2_contents);
    });

    let _ = tokio::join!(h1, h2);
}

fn read_from_file2() -> impl Future<Output=String> {
    async {
        sleep(Duration::new(2, 0));
        println!("{:?}", "Processing file 2");
        String::from("Hello, there from file 2")
    }
}
