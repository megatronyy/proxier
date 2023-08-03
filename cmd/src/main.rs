async fn say_world() {
    println!("hello world");
}

#[tokio::main]
async fn main() {
    let op = say_world();
    op.await?;
}