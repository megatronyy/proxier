#![feature(dropck_eyepatch)]

/*
    本质上move语义的产生很大一部分原因是为了解决内存自动释放的问题。
    或者更加精确地说：move语义保证了变量只被析构一次。
*/
struct Person {
    name: String,
    age: u32,
}

impl Drop for Person {
    fn drop(&mut self) {
        println!("Person Drop: {}", self.name);
    }
}

fn verify() -> bool {
    let person_a = Person {
        name: String::from("Mr. Hello"),
        age: 23,
    };

    {
        let person_b = Person {
            name: String::from("Mr. World"),
            age: 24,
        };
    }

    true
}

fn learn_move() -> bool {
    let person = Person {
        name: String::from("world"),
        age: 24,
    };

    {
        let person_sub = person;
    }

    println!("end of function");

    true
}

//移动语义
fn move_func(person: Person) {
    println!("move_func: {}", person.name);
    // 这里是否应该析构person
}

//借用语义
fn borrow_func(person: &Person) {
    println!("move_func: {}", person.name);
}

fn leave_move_two() -> bool {
    let person = Person {
        name: String::from("world"),
        age: 23,
    };

    {
        let person_sub = person;
        move_func(person_sub);
        println!("end of sub scope");
    }

    println!("end of function");

    true
}

#[test]
fn test_verify() {
    assert!(verify());
}

#[test]
fn test_learn_move() {
    assert!(learn_move());
}

#[test]
fn test_learn_move_two() {
    assert!(leave_move_two());
}

/// drop order test
struct World<'a> {
    inspector: Option<Inspector<'a>>,
    //相当于a: &b
    days: Box<u8>,
}

struct Inspector<'a>(&'a u8);

impl</* #[may_dangle] */'a> Drop for Inspector<'a> {
    fn drop(&mut self) {
        println!("I was only {} days from retirement!", self.0);
    }
}

#[test]
fn test_drop_order() {
    let mut world = World {
        inspector: None,
        days: Box::new(1),
    };

    /***
    编译报错：
     world.inspector = Some(Inspector(&world.days)); // a = &b
    |                                      ^^^^^^^^^^^ borrowed value does not live long enough

    出错的原因是days被inspector borrow了，虽然理论上inpsector先被销毁，
    但是编译器还是认为days 没有strictly outlive inspector，也就是days 没有outlive world。
    是不是有点意外？这是因为，为了完完全全safe，Rust规定如果generic type要安全地实现drop，
    它的generic arguments必须要strictly outlive这个generic type。
    这里不是strictly outlive，因为它们是同一个struct的fields。
    （为什么同一个struct的fields drop有顺序，但是不认为是strictly outelive呢？留给读者思考
     */
    // world.inspector = Some(Inspector(&world.days)); // a = &b
}