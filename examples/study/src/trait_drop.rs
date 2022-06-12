#[warn(unused_imports)]

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
        age: 23
    };

    {
        let person_b = Person {
            name: String::from("Mr. World"),
            age: 24
        };
    }

    true
}

fn learn_move() -> bool {
    let person = Person{
        name: String::from("world"),
        age: 24
    };

    {
        let person_sub = person;
    }

    println!("end of function");

    true
}

//移动语义
fn move_func(person: Person){
    println!("move_func: {}", person.name);
    // 这里是否应该析构person
}

//借用语义
fn borrow_func(person: &Person){
    println!("move_func: {}", person.name);
}

fn leave_move_two() -> bool {
    let person = Person {
        name: String::from("world"),
        age: 23
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
fn test_verify(){
    assert!(verify());
}

#[test]
fn test_learn_move(){
    assert!(learn_move());
}

#[test]
fn test_learn_move_two(){
    assert!(leave_move_two());
}