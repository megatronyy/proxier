/***
&'static T 与 T: 'static的区别
 */


struct Person<'a> {
    name: &'a str,
}

fn foo<T>(_input: &'static T) {
    println!("foo works");
}

fn bar<T: 'static>(_input: &T) {
    println!("bar works");
}

#[test]
fn test_foo() {
    let my_string = String::from("Hello, world!");

    // foo(&my_string);
}

#[test]
fn test_bar() {
    let my_string = String::from("hello, world!");
    bar(&my_string);
}

#[test]
fn test_bar2() {
    let my_string = String::from("hello, world!");

    // my_string的生命周期不是'static，T: 'static限制了struct Person内的引用（name）生命周期
    let my_person = Person { name: my_string.as_str() };
    // bar(&my_person);
}