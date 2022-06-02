#![deny(rust_2018_idioms, clippy::disallowed_methods, clippy::disallowed_types)]
#![forbid(unsafe_code)]

mod struct_tree;
mod lib_smart_ptr;
mod trait_drop;
mod stack_return;
mod lib_cells;
mod lib_threads;
mod lib_mems;
mod trait_dispatch;
mod operator_as;
mod trait_from_into;
mod trait_deref;
mod traits;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
