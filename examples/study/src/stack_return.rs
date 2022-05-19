/*
rust在返回值传递上的最大问题其实是默认栈上分配空间导致的：rust默认所有值都是在栈上的，这些栈上的值在函数返回的时候就需要全部销毁。
如果我们限制函数内部不能创建新的值，那么函数的功能就基本废掉了，我们有无数种逻辑需要在子函数内部创建出新的值，然后返回给上级函数。

这里有2种方式解决这个问题：一是使用Box这样的智能指针在堆上去分配值，然后返回指针。二是能否由函数的调用方分配空间，
子函数内部使用父函数分配好的空间写入。

第一种方式有一定的性能损耗，不那么符合rust在性能上绝不妥协的初衷。第二种方式显然是更好的，
但是我们也不希望程序员手动地在调用前创建一个返回值，然后把返回值的可变引用传入子函数，这样的语法太不舒服了。
聪明的rust编译会帮我们来完成这个事情，
示例代码如下：
*/

struct Bird {
    pub n: i32,
}

impl Drop for Bird {
    fn drop(&mut self) {
        println!("Bird[{}] drop here!", self.n);
    }
}

impl Bird {
    pub fn new() -> Bird {
        Bird {
            n: 42,
        }
    }

    pub fn pass_return(foo: Bird) -> Bird {
        foo;
        return Bird { n: 142 }; //在栈上新生成一个bird并返回
    }
}

#[test]
fn test_bird() {
    let foo1 = Bird::new(); //实际的栈空间是在父函数种分配的
    println!("{}", foo1.n);
    let foo2 = Bird::pass_return(foo1);
}