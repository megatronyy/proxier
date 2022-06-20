#![deny(rust_2018_idioms, clippy::disallowed_methods, clippy::disallowed_types)]
#![forbid(unsafe_code)]

mod name;
mod suffix;

//从mod name里的类型导入了lib，其它rs可以直接通过：use crate::Name的方式引用
pub use self::name::{InvalidName, Name, NameRef};
pub use self::suffix::Suffix;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
