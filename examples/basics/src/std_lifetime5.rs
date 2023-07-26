fn f<'a>() {}
fn g<'a: 'a>(){}

#[test]
fn test_lifetime(){
    let ff = f as fn();
    // let ff = f::<'static> as fn(); //晚绑定函数不可以提交绑定生命周期
    let gg = g::<'static> as fn();
}