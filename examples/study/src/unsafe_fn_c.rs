extern "C" {
    fn abs(input: i32) -> i32;
}

#[test]
fn test_unsafe_fn_c(){
    unsafe {
        println!("abs(-3): {}", abs(-3));

        print!("Hello, world!");
    }
}