use std::cell::{Cell, RefCell, RefMut};
use std::collections::HashMap;
use std::rc::Rc;

struct Foo {
    x: u32,
    y: Cell<u32>,
}

struct Foo2 {
    x: u32,
    y: u32,
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

fn refcell_demo() {
    let shared_map: Rc<RefCell<_>> = Rc::new(RefCell::new(HashMap::new()));

    // create a new block to limit the scope of the dynamic borrow
    {
        let mut map: RefMut<'_, _> = shared_map.borrow_mut();
        map.insert("africa", 92355);
        map.insert("kyoto", 11837);
        map.insert("piccadilly", 11826);
        map.insert("marbles", 38);
    }

    let total: i32 = shared_map.borrow().values().sum();
    println!("{}", total);
}

#[test]
fn test_refcell_demo(){
    refcell_demo();
}