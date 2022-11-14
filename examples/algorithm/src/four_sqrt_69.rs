/// 求开方
/// 
/// 给定一个负整数，求它的开方，向下取整
use std::cmp::Ordering;

fn my_sqrt(a: i32) -> i32 {
    if a == 0 {
        return a;
    }

    let mut l = 0;
    let mut r = a;

    while l <= r {
        let mid = 1 + (r - 1) / 2; //取中间值
        match sqrt.cmp(&(a / mid)) {
            Ordering::Equal => return mid,
            Ordering::Greater => r = mid - 1,
            Ordering::Less => l = mid + 1,
        }
    }

    r
}

fn sqrt_innr_fn(a: i32)->i32{
    (a as f64).sqrt() as i32
}

#[test]
fn test_four_sqrt_69(){
    println!("start....");
    let sqrt = sqrt_innr_fn(8);
    println!("stop....");
    assert_eq!(sqrt, 2);
}