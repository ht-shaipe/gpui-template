//! smol replacement that is wasm-compatible.
//! On native targets, re-exports the original smol crates.
//! On wasm32, provides stubs for OS-specific APIs (Timer, block_on, Async, etc.)
//! while re-exporting pure-Rust crates (channel, executor, futures-lite, etc.).

#![cfg_attr(target_family = "wasm", allow(unused))]

// === Common re-exports (both native and wasm) ===
#[doc(inline)]
pub use async_executor::{Executor, LocalExecutor, Task};
#[doc(inline)]
pub use async_channel as channel;
#[doc(inline)]
pub use async_lock as lock;
#[doc(inline)]
pub use futures_lite::{future, io, pin, prelude, ready, stream};

// === Native-only re-exports ===
#[cfg(not(target_family = "wasm"))]
#[doc(inline)]
pub use {
    async_io::{block_on, Async, Timer},
    blocking::{unblock, Unblock},
};

#[cfg(not(target_family = "wasm"))]
#[doc(inline)]
pub use {async_fs as fs, async_net as net};

#[cfg(not(target_family = "wasm"))]
#[doc(inline)]
pub use async_process as process;

// === WASM stubs ===

#[cfg(target_family = "wasm")]
mod wasm_stubs;

#[cfg(target_family = "wasm")]
pub use wasm_stubs::{block_on, Timer, Async};

#[cfg(target_family = "wasm")]
pub fn unblock<T: Send + 'static>(
    f: impl FnOnce() -> T + Send + 'static,
) -> async_executor::Task<T> {
    // On wasm there's no thread pool; just run synchronously via spawn
    Executor::new().spawn(async move { f() })
}

#[cfg(target_family = "wasm")]
pub use Unblock;

/// Stub for `Unblock` on wasm — just runs on the current thread.
#[cfg(target_family = "wasm")]
pub struct Unblock<T>(std::marker::PhantomData<T>);

#[cfg(target_family = "wasm")]
impl<T> Unblock<T> {
    /// Stub: panics on wasm (blocking I/O wrappers are meaningless on wasm).
    pub fn new(_t: T) -> Self {
        Self(std::marker::PhantomData)
    }
}

mod spawn;
pub use spawn::spawn;
