/*
  借用方的生命周期不能长于出借方的生命周期
*/

#[test]
fn test1() {
    let r;

    {
        let x = 5;
        r = &x;
    }


    println!("r: {}", r);
}

// === function lifetime ===

//fn longest(x: &str, y: &str) -> &str {
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

/// 只需要保证返回的值不会出现悬垂引用
fn get_str<'a>(x: &'a str, y: &str) -> &'a str {
    x
}

#[test]
fn test_longest() {
    let s1 = String::from("hello");
    let s2 = String::from("Rust");

    let s3 = longest(s1.as_str(), s2.as_str());

    println!("return value = {}", s3);
}

// === struct lifetime ===

#[derive(Debug)]
struct A<'a> {
    name: &'a str,
}

#[test]
fn test_struct_lifetime_A() {
    let n = String::from("hello");
    let a = A { name: &n };

    println!("a = {:#?}", a);
}