/*
原生类型使用as操作符进行转换：
展示了u32和u64之间的转换，其他的原生类型也都可以使用as操作符进行转换。
需要注意的是，短（大小）类型转换为长（大小）类型的时候是没有问题的，但是如果反过来，则会被截断处理
*/
#[test]
fn test_as_conv(){
    let a = 1u32;
    let _b = a as u64;
    let c = 3u64;
    let _d = c as u32;
}

#[test]
fn test_as_cutoff(){
    let a = std::u32::MAX; // 4294967295
    let b = a as u16;
    assert_eq!(b, 65535);

    let e = -1i32;
    let f = e as u32;
    println!("{:?}", e.abs()); // 1
    println!("{:?}", f); // 4294967295
}

struct S(i32);
trait A {
    fn test(&self, i: i32);
}

trait B {
    fn test(&self, i: i32);
}

impl A for S {
    fn test(&self, i: i32) {
        println!("From A: {:?}", i);
    }
}

impl B for S {
    fn test(&self, i: i32) {
        println!("From B: {:?}", i + 1);
    }
}

/*
为结构体实现多个trait时出现同名方法的情况
*/

#[test]
fn test_as_trait(){
    let s = S(1);
    A::test(&s, 1);
    B::test(&s, 1);

    <S as A>::test(&s, 1);
    <S as B>::test(&s, 1);
}