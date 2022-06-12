#![deny(rust_2018_idioms, clippy::disallowed_methods, clippy::disallowed_types)]
#![forbid(unsafe_code)]

mod struct_tree;
mod std_smart_ptr;
mod trait_drop;
mod stack_return;
mod std_cells;
mod std_threads;
mod std_mems;
mod trait_dispatch;
mod operator_as;
mod trait_from_into;
mod trait_deref;
mod std_macros;
mod lib_tokio;
mod std_lifetime;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
