use std::future::Future;
use std::pin::Pin;
use std::sync::{Arc, Mutex};
use std::task::{Context, Poll, Waker};
use std::time::Duration;
use std::thread;

pub struct TimerFuture {
    share_state: Arc<Mutex<SharedState>>,
}

struct SharedState {
    completed: bool,
    waker: Option<Waker>,
}

impl Future for TimerFuture {
    type Output = ();

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        let mut share_state = self.share_state.lock().unwrap();
        if share_state.completed {
            Poll::Ready(())
        } else {
            share_state.waker = Some(cx.waker().clone());
            Poll::Pending
        }
    }
}

impl TimerFuture {
    pub fn new(duration: Duration) -> Self {
        let share_state = Arc::new(Mutex::new(
            SharedState {
                completed: false,
                waker: None,
            }
        ));
        let thread_share_state = share_state.clone();
        thread::spawn(move || {
            thread::sleep(duration);
            let mut share_state = thread_share_state.lock().unwrap();
            share_state.completed = true;
            if let Some(waker) = share_state.waker.take() {
                waker.wake();
            }
        });
        TimerFuture { share_state }
    }
}