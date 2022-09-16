fn return_str<'a>() -> &'a str {
    let mut s: String = "Rust".to_string();
    for i in 0..3 {
        s.push_str("Good "); // "RustGood Good Good "
    }
    &s[..]
}

fn main() {
    let x = return_str();
}