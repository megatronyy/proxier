/// 贪心算法--分糖果
/// 
/// 初始所有人糖果为1
/// 先从右往左遍历，索引1开始，相邻的比较
/// 遍历完成后，用一个变量ans保存最后一个元素的值，因为他的数不会再变
/// 然后再从左往右遍历，从索引倒数第二位开始，右边与左边比大小，设置糖果数
/// 变量ans与每个位置的糖果数相加
/// 然后返回ans
pub fn candy(ratings: Vec<i32>) -> i32 {

    let total = ratings.len();
    if total < 2 {
        return total as i32;
    }

    let mut candys = vec![1; total];
    for i in 1..ratings.len() {
        if ratings[i] > ratings[i-1] {
            candys[i] = candys[i - 1] + 1;
        }
    }

    for i in (0..ratings.len() - 1).rev() {
        if ratings[i] > ratings[i+1] {
            candys[i] = candys[i].max(candys[i+1] + 1);
        }
    }
    candys.iter().sum::<i32>() as i32
}

#[test]
fn test_candy(){
    let ans = candy(vec![1, 0, 2]);

    println!("ans:{}", ans);

    assert_eq!(ans, 5);
}