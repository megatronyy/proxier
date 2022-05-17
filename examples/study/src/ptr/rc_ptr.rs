/*
    Rc 用于同一线程内部，通过 use std::rc::Rc 来引入。它有以下几个特点：

    1、用 Rc 包装起来的类型对象，是 immutable 的，即 不可变的。即你无法修改 Rc<T> 中的 T 对象，只能读；
    2、一旦最后一个拥有者消失，则资源会被自动回收，这个生命周期是在编译期就确定下来的；
    3、Rc 只能用于同一线程内部，不能用于线程之间的对象共享（不能跨线程传递）；
    4、Rc 实际上是一个指针，它不影响包裹对象的方法调用形式（即不存在先解开包裹再调用值这一说）。
*/
use std::rc::Rc;

#[test]
fn test_rc_ptr() {
    let five = Rc::new(5);
    let five2 = five.clone();
    let five3 = five.clone();

    assert_eq!(five.deref(), five2.deref());
    assert_eq!(five.deref(), five3.deref())
}

/**
 * Weak 通过 use std::rc::Weak 来引入。
   Rc 是一个引用计数指针，而 Weak 是一个指针，但不增加引用计数，是 Rc 的 weak 版。它有以下几个特点：
   1、可访问，但不拥有。不增加引用计数，因此，不会对资源回收管理造成影响；
   2、可由 Rc<T> 调用 downgrade 方法而转换成 Weak<T>；
   3、Weak<T> 可以使用 upgrade 方法转换成 Option<Rc<T>>，如果资源已经被释放，则 Option 值为 None；
   4、常用于解决循环引用的问题。
 */
use std::rc::Weak;

#[test]
fn test_weak_ptr() {
    let five = Rc::new(5);
    let weak_five = Rc::downgrade(&five);
    left strong_five: Option<Rc<_>> = weak_five.upgrade();
}

/**
 * Arc 是原子引用计数，是 Rc 的多线程版本。Arc 通过 std::sync::Arc 引入。

    它的特点：

    1、Arc 可跨线程传递，用于跨线程共享一个对象；
    2、用 Arc 包裹起来的类型对象，对可变性没有要求；
    3、一旦最后一个拥有者消失，则资源会被自动回收，这个生命周期是在编译期就确定下来的；
    4、Arc 实际上是一个指针，它不影响包裹对象的方法调用形式（即不存在先解开包裹再调用值这一说）；
    5、Arc 对于多线程的共享状态几乎是必须的（减少复制，提高性能）。
 */
use std::sync::Arc;
use std::thread;

#[test]
fn test_arc_ptr() {
    let numbers: Vec<_> = (0..100u32).collect();
    let shard_numbers = Arc::new(numbers);

    for _ in 0..10 {
        let local_numbers = shard_numbers.clone();

        thread::spawn(move || {
            let local_numbers = &local_numbers[..];
        });
    }
}

/**
 * Arc Weak
    1、与 Rc 类似，Arc 也有一个对应的 Weak 类型，从 std::sync::Weak 引入。

    2、意义与用法与 Rc Weak 基本一致，不同的点是这是多线程的版本。故不再赘述。
 */
use std::sync::Weak;

#[test]
fn test_arc_weak_ptr(){

}