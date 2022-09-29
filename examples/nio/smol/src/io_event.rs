use std::io::{self, Read, Write};
#[cfg(windows)]
use std::net::SocketAddr;
use std::sync::atomic::{self, Ordering};
use std::sync::Arc;

#[cfg(not(target_os = "linux"))]
use socket2::{Domain, Socket, Type};

pub(crate) struct IoEvent {}

impl IoEvent {
    pub fn new() -> io::Result<IoEvent> {
        Ok(IoEvent {})
    }
}