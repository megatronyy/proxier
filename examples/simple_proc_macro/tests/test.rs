/*
过程宏实现三种类型的宏：
1、自定义派生属性，可以自定义类似于#[derive(Debug)]这样的derive属性，
   可以自动为结构体或枚举类型进行语法扩展。
2、自定义属性，可以自定义类似于#[cfg()]这种属性
3、Bang宏，和macro_rules!定义的宏类似，以Bang符号（就是叹号"!"）结尾
  的宏，可以像函数一样被调用
*/

// 测试自定义派生宏
#[macro_use]
extern crate simple_proc_macro;

#[derive(A)]
struct A;

#[test]
fn test_derive_a() {
    assert_eq!("hello from impl A".to_string(), A.a());
}


// 测试自定义属性宏
use simple_proc_macro::attr_with_args;

#[attr_with_args("Hello, Rust!")]
fn foo() {}

#[test]
fn test_foo() {
    assert_eq!(foo(), "Hello, Rust!");
}

// 测试Bang宏
use simple_proc_macro::hashmap;

#[test]
fn test_hashmap() {
    let hm = hashmap! { "a": 1, "b": 2, };
    assert_eq!(hm["a"], 1);
    let hm = hashmap! { "a" => 1, "b" => 2, "c" => 3 };
    assert_eq!(hm["d"], 4);
}