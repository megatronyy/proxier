use std::any::Any;
use std::io;

/*
静态生命周期的类型都实现了Any trait
非静态生命周期的未实现
*/
fn print_for_string(string: Box<dyn Any>) {
    if let Ok(string) = string.downcast::<String>() {
        println!("String (length {}): {}", string.len(), string);
    } else {
        println!("Not String");
    }
}

#[tokio::main]
async fn main() {
    let my_string = "Hello World".to_string();
    print_for_string(Box::new(my_string));
    print_for_string(Box::new(0i8));
}