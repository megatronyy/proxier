struct S;

//Return short ('b) reference
fn f1sr<'b, 'a>(rb: &'b &'a S) -> &'b S {
    // 含有下面隐藏的关系
    // &'b &'a S => 'a: 'b    'b依赖'a
    // &'b T => T: 'b
    *rb
}

fn f2sr<'b, 'a>(rb: &'b &'a mut S) -> &'b S{
    *rb
}

fn f3sr<'b, 'a>(rb: &'b mut &'a S) -> &'b S{
    *rb // &'c S
}

fn f4sr<'b, 'a>(rb: &'b mut &'a mut S) -> &'b S{
    *rb // &'c mut S => 'c: 'b //涉及到reborrow
}

#[test]
fn test_lifetime() {

}