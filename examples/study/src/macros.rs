// 声明式宏
macro_rules! match_let {
    () => {
        println!("empty tokens matched")
    };
    (let v: u32;) => {
        println!("matched: `let v: u32;`")
    };
}

#[test]
fn test_match_let(){
    match_let!();
    match_let!(let v: u32;);
}