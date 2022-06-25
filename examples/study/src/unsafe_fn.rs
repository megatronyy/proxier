/// 5、调用不安全的函数或方法
unsafe fn dangerous() {
    println!("do something dangerous");
}

fn foo() {
    let mut num = 5;
    let r1 = &num as *const i32;
    let r2 = &mut num as *mut i32;
    unsafe {
        println!("*r1 = {}", *r1);
        println!("*r2 = {}", *r2);
    }
}

#[test]
fn test_unsafe_fn(){
    unsafe {
        dangerous();
    }
    //dangerous(); //error
    foo();

    println!("Hello, world!");
}