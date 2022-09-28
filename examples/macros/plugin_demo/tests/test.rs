#![feature(plugin)]
#![plugin(plugin_demo)]
#[test]
fn test_plugin() {
    assert_eq!(roman_to_digit!(MMXVIII), 2018)
}