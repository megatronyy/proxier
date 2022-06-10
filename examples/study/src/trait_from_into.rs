/*
对于类型T，如果它实现了From＜U＞，则可以通过T：：from（u）来生成T类型的实例，此处u为U的类型实例。
*/

#[test]
fn test_trait_from() {
    let string = "hello".to_string();
    let other_string = String::from("hello");
    assert_eq!(string, other_string)
}

/*
对于类型T，如果它实现了Into＜U＞，则可以通过into方法来消耗自身转换为类型U的新实例。
*/
#[derive(Debug)]
struct Person {
    name: String,
}

impl Person {
    fn new<T: Into<String>>(name: T) -> Person {
        Person { name: name.into() }
    }
}

#[test]
fn test_trait_into() {
    let _person = Person::new("Alex");
    let person = Person::new("Alex".to_string());
    println!("{:?}", person);
}