/**
迭代器分类：
1、外部迭代器
2、内部迭代器
3、都在std::iter模块中
 */

// === 自定义 MyVec ===

struct MyVec(Vec<i32>);

impl MyVec {
    fn new() -> MyVec {
        MyVec(Vec::new())
    }

    fn add(&mut self, elem: i32) {
        self.0.push(elem);
    }
}

impl FromIterator<i32> for MyVec {
    fn from_iter<T: IntoIterator<Item=i32>>(iter: T) -> Self {
        let mut c = MyVec::new();
        for i in iter {
            c.add(i);
        }
        c
    }
}

// === 自定义 MyVec 测试 ===
#[test]
fn test_myvec() {
    let iter = (0..5).into_iter();
    let c = MyVec::from_iter(iter);
    assert_eq!(c.0, vec![0, 1, 2, 3, 4]);

    let iter = (0..5).into_iter();
    let c: MyVec = iter.collect();
    assert_eq!(c.0, vec![0, 1, 2, 3, 4]);

    let iter = (0..5).into_iter();
    let c = iter.collect::<MyVec>();
    assert_eq!(c.0, vec![0, 1, 2, 3, 4]);
}

// === 迭代器练习 ===
struct Counter {
    count: usize,
}

impl Iterator for Counter {
    type Item = usize;

    fn next(&mut self) -> Option<Self::Item> {
        self.count += 1;
        if self.count < 6 {
            Some(self.count)
        } else {
            None
        }
    }
}

#[test]
fn test_iter_next() {
    let mut counter = Counter { count: 0 };
    assert_eq!(Some(1), counter.next());
    assert_eq!(Some(2), counter.next());
    assert_eq!(Some(3), counter.next());
    assert_eq!(Some(4), counter.next());
    assert_eq!(Some(5), counter.next());
    assert_eq!(None, counter.next());
}

/**
size_hint方法：
1、返回类型是一个元组(usize, Option<usize>),此元组表示迭代器剩余长度的边界信息。
   元组中第一个元素表示下限（lower bound）,第二个元素表示上限。
 */
#[test]
fn test_iter_size_hint() {
    let a: [i32; 3] = [2, 4, 6];
    let mut iter = a.iter();
    assert_eq!((3, Some(3)), iter.size_hint());
    iter.next();
    assert_eq!((2, Some(2)), iter.size_hint());
}

/**

for是语法糖，展开后，等价于下面的代码
简单的说，for循环就是利用迭代器模式实现的一个语法糖，它属于外部迭代器

#[test]
fn test_iterator() {
    let v = vec![1, 2, 3, 4, 5];
    {
        let mut _iterator = v.into_iter();
        loop {
            match _iterator.next() {
                Some(i) => {
                    println!("{}", i);
                }
                None => break,
            }
        }
    }
}
 */
#[test]
fn test_for() {
    let v = vec![1, 2, 3, 4, 5];
    for i in v {
        println!("{}", i);
    }
}

/**
迭代器适配器
在软件世界中，通过适配器模式同样可以将一个接口转换成所需的另一个接口
适配器还有一个列名，包装器(Wrapper)
迭代器有个map方法，返回Map struct，这个Map就是迭代器适配器。
Map、Chain、Cloned、Cycle、，Enumerate、Filter、FlatMap、FilterMap、Fuse、Rev等
 */
fn test_iter_wapper() {
    let a = [1, 2, 3];
    let mut iter = a.into_iter().map(|x| 2 * x);
    assert_eq!(iter.next(), Some(2));
    assert_eq!(iter.next(), Some(4));
    assert_eq!(iter.next(), Some(6));
    assert_eq!(iter.next(), None);
}

/**
迭代器消费器，Rust中的迭代器都是惰性的，也就是说，
他们不会自动发生遍历行为，除非调用next方法去
消费其中的数据。除了用for直接消费迭代大数据，
还提供了一个其它方法消费迭代器内数据，它们叫作消费器(Consumer)。
any、fold、collect等
 */
#[test]
fn test_iter_consumer() {
    let a = [1, 2, 3];
    assert_eq!(a.iter().any(|&x| x != 2), true);
    let sum = a.iter().fold(0, |acc, x| acc + x);
    assert_eq!(sum, 6);
}

/// 自定义迭代器适配器
#[derive(Clone, Debug)]
pub struct Step<I> {
    iter: I,
    skip: usize,
}

impl<I> Iterator for Step<I>
    where I: Iterator
{
    type Item = I::Item;

    fn next(&mut self) -> Option<Self::Item> {
        let elt = self.iter.next();
        if self.skip > 0 {
            self.iter.nth(self.skip - 1);
        }
        elt
    }
}

/// 生成Step适配器
pub fn step<I>(iter: I, skip: usize) -> Step<I>
    where I: Iterator
{
    assert!(skip != 0);
    Step {
        iter: iter,
        skip: skip - 1,
    }
}

/// 为所有的适配器实现step方法
pub trait IterExt: Iterator {
    fn step(self, n: usize) -> Step<Self>
        where Self: Sized
    {
        step(self, n)
    }
}

impl<T: ?Sized> IterExt for T where T: Iterator {}

#[test]
fn test_iter_ext() {
    let arr = [1, 2, 3, 4, 5, 6];
    let sum = arr.iter().step(2).fold(0, |acc, x| acc + x);
    assert_eq!(9, sum);
}
