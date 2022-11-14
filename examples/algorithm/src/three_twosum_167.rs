/// 指针类算法
/// 
/// 增序的数组，两数之和

use std::cmp::Ordering;

fn two_sum(numbers: Vec<i32>, target: i32) -> Vec<i32> {
    let(mut a, mut b) = (0, numbers.len() -1);

    loop {
        match target.cmp(&(numbers[a] + numbers[b])) {
            Ordering::Equal => return vec![a as i32 + 1, b as i32 + 1],
            Ordering::Greater => a += 1,
            Ordering::Less => b -= 1,
        }
    }
}

#[test]
fn test_two_sum(){
    let numbers = vec![2,7,11,15];
    let target = 9;

    assert_eq!(two_sum(numbers, target), vec![1, 2]);
}