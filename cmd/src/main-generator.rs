#![feature(generators, generator_trait)]

use std::pin::Pin;

fn main() {
    let mut gen = || {
        println!("执行第1次yield");
        yield 1;

        println!("执行第2次yield");
        yield 2;

        println!("执行第3次yield");
        yield 3;

        println!("执行第4次yield");
        yield 4;
    };

    let c = Pin::new(&mut gen).resume(());
    println!("{:?}", c);

    let c = Pin::new(&mut gen).resume(());
    println!("{:?}", c);

    let c = Pin::new(&mut gen).resume(());
    println!("{:?}", c);

    let c = Pin::new(&mut gen).resume(());
    println!("{:?}", c);
}