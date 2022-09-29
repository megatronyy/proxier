#[cfg(not(any(
target_os = "linux",     // epoll
target_os = "android",   // epoll
target_os = "illumos",   // epoll
target_os = "macos",     // kqueue
target_os = "ios",       // kqueue
target_os = "freebsd",   // kqueue
target_os = "netbsd",    // kqueue
target_os = "openbsd",   // kqueue
target_os = "dragonfly", // kqueue
target_os = "windows",   // wepoll
)))]
compile_error!("reactor does not support this target OS");

use std::collections::BTreeMap;
use std::io;
use std::os::solid::io::RawFd;
use std::os::windows::io::RawSocket;
use std::sync::Arc;
use std::task::Waker;
use std::time::Instant;
use concurrent_queue::ConcurrentQueue;
use once_cell::sync::Lazy;
use slab::Slab;
#[cfg(windows)]
use socket2::Socket;
use crate::io_event::IoEvent;

#[cfg(unix)]
use crate::sys::fcntl::{fcntl, FcntlArg};

// use crate::io_

pub(crate) struct Reactor {
    /// Raw bindings to epoll/kqueue/wepoll.
    sys: sys::Reactor,

    /// Registered sources
    sources: piper::Mutex<Slab<Arc<Source>>>,

    /// Temporary storage for I/O events when polling the reactor.
    events: piper::Mutex<sys::Events>,

    /// An ordered map of registered timers.
    ///
    /// Timers are in the order in which they fire. The `usize` in this type is a timer ID used to
    /// distinguish timers that fire at the same time. The `Waker` represents the task awaiting the
    /// timer.
    timers: piper::Mutex<BTreeMap<(Instant, usize), Waker>>,

    /// A queue of timer operations (insert and remove).
    ///
    /// When inserting or removing a timer, we don't process it immediately - we just push it into
    /// this queue. Timers actually get processed when the queue fills up or the reactor is polled.
    timer_ops: ConcurrentQueue<TimerOp>,

    /// An I/O event that is triggered when a new timer is registered.
    ///
    /// The reason why this field is lazily created is because `IoEvent`s can be created only after
    /// the reactor is fully initialized.
    timer_event: Lazy<IoEvent>,
}

impl Reactor {
    pub fn get() -> &'static Reactor {
        static REACTOR: Lazy<Reactor> = Lazy::new(|| Reactor {
            sys: sys::Reactor::new().expect("cannot initialize I/O event notification"),
            sources: piper::Mutex::new(Slab::new()),
            events: piper::Mutex::new(sys::Events::new()),
            timers: piper::Mutex::new(BTreeMap::new()),
            timer_ops: ConcurrentQueue::bounded(1000),
            timer_event: Lazy::new(|| IoEvent::new().expect("cannot create an `IoEvent`")),
        });
        &REACTOR
    }
}

/// A single timer operation.
enum TimerOp {
    Insert(Instant, usize, Waker),
    Remove(Instant, usize),
}

pub(crate) struct Source {
    // Raw file descriptor on Unix platforms.
    #[cfg(unix)]
    pub(crate) row: RawFd,

    /// Raw socket handle on Windows.
    #[cfg(windows)]
    pub(crate) raw: RawSocket,

    /// The key of this source obtained during registration
    key: usize,

    /// Tasks interested in events on a source.
    wakers: piper::Mutex<Wakers>,
}

#[derive(Debug)]
struct Wakers {
    /// Tasks waiting for the next readability event.
    readers: Vec<Waker>,

    /// Tasks waiting for the next writability event.
    writers: Vec<Waker>,
}

impl Source {
    pub(crate) fn reregister_io_event(&self) ->
    io::Result<()> {
        let wakers = self.wakers.lock();
        Ok(())
    }
}

/// Raw bindings to wepoll (Windows)
#[cfg(target_os = "windows")]
mod sys {
    use std::io;
    use std::os::windows::io::{AsRawSocket, RawSocket};
    use std::time::Duration;
    use wepoll_binding::{Epoll, EventFlag};

    pub struct Reactor(Epoll);

    impl Reactor {
        pub fn new() -> io::Result<Reactor> {
            Ok(Reactor(Epoll::new()?))
        }

        pub fn register(&self, sock: RawSocket, key: usize) -> io::Result<()> {
            self.0.register(&As(sock), EventFlag::empty(), key as u64)
        }

        pub fn reregister(
            &self,
            sock: RawSocket,
            key: usize,
            read: bool,
            write: bool,
        ) -> io::Result<()> {
            let mut flags = EventFlag::ONESHOT;
            if read {
                flags |= read_flags();
            }

            if write {
                flags |= write_flags();
            }

            self.0.reregister(&As(sock), flags, key as u64)
        }
        pub fn deregister(&self, sock: RawSocket) -> io::Result<()> {
            self.0.deregister(&As(sock))
        }

        pub fn wait(&self, events: &mut Events, timeout: Option<Duration>)
                    -> io::Result<usize> {
            let timeout = timeout.map(|t| {
                if t == Duration::from_millis(0) {
                    t
                } else {
                    t.max(Duration::from_millis(1))
                }
            });
            events.0.clear();
            self.0.poll(&mut events.0, timeout)
        }
    }

    struct As(RawSocket);

    impl AsRawSocket for As {
        fn as_raw_socket(&self) -> RawSocket {
            self.0
        }
    }

    fn read_flags() -> EventFlag {
        EventFlag::IN | EventFlag::RDHUP
    }

    fn write_flags() -> EventFlag {
        EventFlag::OUT
    }

    pub struct Events(wepoll_binding::Events);

    impl Events {
        pub fn new() -> Events {
            Events(wepoll_binding::Events::with_capacity(1000))
        }

        pub fn iter(&self) -> impl Iterator<Item=Event> + '_ {
            self.0.iter().map(|ev| Event {
                readable: ev.flags().intersects(read_flags()),
                writeable: ev.flags().intersects(write_flags()),
                key: ev.data() as usize,
            })
        }
    }

    pub struct Event {
        pub readable: bool,
        pub writeable: bool,
        pub key: usize,
    }
}