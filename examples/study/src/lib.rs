#![deny(rust_2018_idioms, clippy::disallowed_methods, clippy::disallowed_types)]
#![forbid(unsafe_code)]

mod tree;
mod rc_ptr;
mod destruct;
mod stack_return;
mod cells;
mod threads;
mod mems;
mod traits;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
