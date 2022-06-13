#![deny(rust_2018_idioms, clippy::disallowed_methods, clippy::disallowed_types)]
#![forbid(unsafe_code)]

use tokio::sync::mpsc;
pub use tracing::{debug, error, info, warn};

mod rt;

fn main() {
    // let (shutdown_tx, mut shutdow_rx) = mpsc::unbounded_channel();
    // let bind =BindT

    tokio::select! {

    }
}
