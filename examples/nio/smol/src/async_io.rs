//! Abstraction over [epoll]/[kqueue]/[wepoll].
//!
//! [epoll]: https://en.wikipedia.org/wiki/Epoll
//! [kqueue]: https://en.wikipedia.org/wiki/Kqueue
//! [wepoll]: https://github.com/piscisaureus/wepoll

use std::sync::Arc;
use crate::reactor::Source;

pub struct Async<T> {
    source: Arc<Source>,
    io: Option<Box<T>>,
}