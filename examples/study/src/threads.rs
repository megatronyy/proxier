// use std::thread;
//
// #[test]
// fn test_thread(){
//     let mut data = vec![1, 2, 3];
//     for i in 0..3 {
//         thread::spawn(move || {
//             data[i] += 1;
//         });
//     }
//
//
//     thread::sleep_ms(50);
//     //println!("{:?}", data)
// }

fn foo<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() % 2 == 0 {
        x
    } else {
        y
    }
}

#[test]
fn test_foo(){
    let x = String::from("hello");
    let z;
    let y = String::from("world");
    z = foo(&x, &y);
    println!("{:?}", z);
}