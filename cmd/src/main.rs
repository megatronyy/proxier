async fn say_world() {
    println!("hello world");
}

fn main() {
    let op = say_world();
    op.await;
}