mod timer;
mod executor;

#[cfg(test)]
mod tests {
    use std::time::Duration;
    use crate::executor::new_executor_and_spawner;
    use crate::timer::TimerFuture;

    #[test]
    fn test_timer_future() {
        let (executor, spawner) = new_executor_and_spawner();
        spawner.spawn(async {
            println!("howdy!");
            TimerFuture::new(Duration::new(2, 0)).await;
            println!("done");
        });

        drop(spawner);

        executor.run();
    }
}
