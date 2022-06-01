#[derive(Debug)]
struct Foo;

trait Bar {
    fn baz(&self);
}

impl Bar for Foo {
    fn baz(&self) {
        println!("{:?}", self)
    }
}

fn static_dispatch<T>(t: &T) where T: Bar {
    t.baz();
}

/*
    trait本身也是一种类型，但它的类型大小在编译期是无法确定的，
    所以trait对象必须使用指针。
    可以利用引用操作符&或Box＜T＞来制造一个trait 对象。

    trait object trait对象，动态分发，有虛表
    impl trait 静态分发，性能更高
    dyn trait 动态分发

    为什么编译器会提示 Box<Trait> 会被废弃, 特地引入了 dyn 关键字呢
    RFC-2113 明确说明了引入 dyn 的原因, 即语义模糊, 令人困惑,
    原因在于没有 dyn 让 Trait 和 trait objects 看起来完全一样, RFC 列举了３个例子说明.

    trait这种对行为约束的特性也非常适合作为类型的标签

    Rust一共提供了5个重要的标签trait，都被定义在标准库std：：marker模块中。它们分别是：
    · Sized trait，用来标识编译期可确定大小的类型。
    · Unsize trait，目前该trait为实验特性，用于标识动态大小类型（DST）。
    · Copy trait，用来标识可以按位复制其值的类型。
    · Send trait，用来标识可以跨线程安全通信的类型。
    · Sync trait，用来标识可以在线程间安全共享引用的类型。
*/
fn dynamic_dispatch(t: &dyn Bar){
    t.baz();
}

#[test]
fn test_traits(){
    let foo = Foo;
    static_dispatch(&foo);
    dynamic_dispatch(&foo);
}