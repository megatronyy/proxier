use std::cell::Cell;

struct Foo {
    x: u32,
    y: Cell<u32>,
}

struct Foo2{
    x: u32,
    y: u32
}

#[test]
fn test_foo() {
    let foo = Foo { x: 1, y: Cell::new(3) };
    assert_eq!(1, foo.x);
    assert_eq!(3, foo.y.get());
    foo.y.set(5);
    assert_eq!(5, foo.y.get());
}

#[test]
fn test_foo2() {
    let foo = Foo2 { x: 1, y: 3 };
    assert_eq!(1, foo.x);
    assert_eq!(3, foo.y);
}