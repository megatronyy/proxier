struct S;

// Return long ('a) reference
fn f1lr<'b, 'a>(rb: &'b &'a S) -> &'a S {
    *rb
}

// fn f2lr<'b, 'a>(rb: &'b &'a mut S) -> &'a S {
//     *rb // &'c mut S => 'c: 'b, 'a: 'b
// }

// fn f3lr<'b, 'a>(rb: &'b mut &'a S) -> &'a S {
//     *rb // &'c mut S => 'c: 'b, 'a: 'b
// }
//
// fn f4lr<'b, 'a>(rb: &'b mut &'a mut S) -> &'a S {
//     *rb // &'c mut S => 'c: 'b, 'a: 'b
// }

#[test]
fn test_lifetime(){

}

