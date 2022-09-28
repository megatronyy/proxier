struct Cacher<T>
    where T: Fn(u32) -> u32
{
    calcuation: T,
    value: Option<u32>,
}

impl<T> Cacher<T> where T: Fn(u32) -> u32 {
    fn new(calcuation: T) -> Cacher<T> {
        Cacher {
            calcuation,
            value: None,
        }
    }

    fn value(&mut self, arg: u32) -> u32 {
        match self.value {
            Some(v) => v,
            None => {
                let v = (self.calcuation)(arg);
                self.value = Some(v);
                v
            }
        }
    }
}

#[test]
fn test_cacher() {
    let mut c = Cacher::new(|x| x + 1);

    let v1 = c.value(1);
    println!("v1 = {}", v1);

    let v2 = c.value(2);
    println!("v2 = {}", v2);

    assert_eq!(v1, v2);
}

// #![feature(core_intrinsics)]
// fn print_type_of<T>(_: T){
//     println!("{}", unsafe { std::intrinsics::type_name::<T>() });
// }

#[test]
fn test_fnonce(){
    let i = vec![1];
    let x: Box<dyn FnOnce() -> ()> = Box::new(move || {
        println!("{:?}", i);
    });
    x();
    // println!("{:?}", i);
}