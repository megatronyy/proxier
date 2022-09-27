use lazy_static::lazy_static;
use regex::Regex;
use syn::UseRename;

lazy_static! {
    static ref USERNAME: Regex = {
        println!("Compiling username regex...");
        Regex::new("^[a-z0-9_-]{3,16}$").unwrap();
    };
}

#[test]
fn test_lazy_static() {
    println!("Let's validate some usernames");
    validate("fergie");
    validate("will.i.am")
}

fn validate(name: &str) {
    println!("is_match({:?}): {}", name, USERNAME.is_match(name));
}