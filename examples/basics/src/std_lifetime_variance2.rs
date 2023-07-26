// struct Interface<'a> {
//     manager: &'a mut Manager<'a>, //此处代码有坏味道
// }

struct Interface<'r, 'val> {
    manager: &'r mut Manager<'val>, // 'val: 'r
}

impl Interface<'_, '_> {
    pub fn noop(self) {
        println!("interface consumed")
    }
}

struct Manager<'val> {
    text: &'val str,
}

struct List<'val> {
    manager: Manager<'val>,
}

impl<'val> List<'val> {
    pub fn get_interface<'r>(&'r mut self) -> Interface<'r, 'val> {
        Interface {
            manager: &mut self.manager,
        }
    }
}

fn use_list(list: &List<'_>) {
    println!("{}", list.manager.text);
}

#[test]
fn test_lifetime3() {
    let mut list = List {
        manager: Manager {
            text: "hello",
        }
    };

    list.get_interface().noop();

    println!("Interface should be dropped here and the borrow released");

    // 下面的调用会失败，因为同时有不可变/可变借用
    // 但是Interface在之前调用完成后就应该被释放了
    use_list(&list);
}