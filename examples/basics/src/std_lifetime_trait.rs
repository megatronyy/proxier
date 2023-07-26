use std::future::Future;
use async_trait::async_trait;

#[async_trait]
trait Example24 {
    async fn fetch(trace_id: &str, span_id: &str);
}

// fn fetch<'a, 'b, 'c>(trace_id: &'a str, span_id: &'b str) -> Box<dyn Future<Output=()> + 'c>
//     where 'a: 'c, 'b: 'c
// {
//     Box::new(async move {
//         println!("{}", trace_id);
//         print!("{}", span_id);
//     })
// }

#[test]
fn test_lifetime_trait() {}