struct S;

// Return short (’b) mutable reference
// M
// fn f1sm<'b, 'a>(rb: &'b &'a S) -> &'b mut S {
//     *rb
// }

// M
// fn f2sm<'b, 'a>(rb: &'b &'a mut S) -> &'b mut S {
//     *rb
// }

// M
// fn f3sm<'b, 'a>(rb: &'b mut &'a S) -> &'b mut S {
//     *rb
// }
//
// // M
fn f4sm<'b, 'a>(rb: &'b mut &'a mut S) -> &'b mut S {
    *rb
}

#[test]
fn test_lifetime(){

}