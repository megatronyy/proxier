/*
  借用方的生命周期不能长于出借方的生命周期
*/

#[test]
fn test1() {
    let r;

    {
        let x = 5;
        r = &x;
    }


    println!("r: {}", r);
}