use std::ops::Deref;
use std::rc::Rc;

/*
自动解引用,Rc并没有实现chars()方法，x使用Deref自动解引用，直接调用str的chars()方法
*/
#[test]
fn test_auto_deref() {
    let x = Rc::new("hello");
    println!("{:?}", x.chars())
}

/*
手动解引用
clone方法在Rc和&str类型中都被实现了，所以调用时会直接调用Rc的clone方法，
如果想调用Rc里面&str类型的clone方法，则需要使用“解引用”操作符手动解引用。
*/
#[test]
fn test_manul_deref() {
    let x = Rc::new("hello");
    let y = x.clone(); //Rc<&str>
    let z = (*x).clone(); // &str 手动解引用

    println!("{:?}", y);
    println!("{:?}", z);
}

/*
只能通过手动解引用把&String类型转换成&str类型，具体有下列几种方式。
· match x.deref（），直接调用deref方法，需要use std：：ops：：Deref。
· match x.as_ref（），String类型提供了as_ref方法来返回一个&str类似，该方法定义于AsRef trait中。
· match x.borrow（），方法borrow定义于Borrow trait中，行为和AsRef类型一样。需要use std：：borrow：：Borrow。
· match &*x，使用“解引用”操作符，将String转换为str，然后再用“引用”操作符转为&str。
· match &x[..]，这是因为String类型的index操作可以返回&str类型。
*/
#[test]
fn test_match_deref() {
    let x = "hello".to_string();
    match x.deref() {
        "hello" => { println!("hello") }
        _ => {}
    }
}