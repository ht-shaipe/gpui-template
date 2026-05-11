//! Spawn a future onto the global executor.
//!
//! On native, uses a multi-threaded executor with `async_io::block_on`.
//! On wasm, uses a single-threaded executor.

use std::future::Future;
use async_executor::{Executor, Task};
use async_lock::OnceCell;

/// Spawns a task onto the global executor.
#[cfg(not(target_family = "wasm"))]
pub fn spawn<T: Send + 'static>(future: impl Future<Output = T> + Send + 'static) -> Task<T> {
    use std::panic::catch_unwind;
    use std::thread;
    use async_io::block_on;
    use futures_lite::future;

    static GLOBAL: OnceCell<Executor<'_>> = OnceCell::new();

    fn global() -> &'static Executor<'static> {
        GLOBAL.get_or_init_blocking(|| {
            let num_threads = {
                std::env::var("SMOL_THREADS")
                    .ok()
                    .and_then(|s| s.parse().ok())
                    .unwrap_or(1)
            };

            for n in 1..=num_threads {
                thread::Builder::new()
                    .name(format!("smol-{}", n))
                    .spawn(|| loop {
                        catch_unwind(|| block_on(global().run(future::pending::<()>()))).ok();
                    })
                    .expect("cannot spawn executor thread");
            }

            let ex = Executor::new();
            #[cfg(not(target_os = "espidf"))]
            ex.spawn(async_process::driver()).detach();
            ex
        })
    }

    global().spawn(future)
}

/// Spawns a task onto the global executor (wasm single-threaded version).
#[cfg(target_family = "wasm")]
pub fn spawn<T: Send + 'static>(future: impl Future<Output = T> + Send + 'static) -> Task<T> {
    static GLOBAL: OnceCell<Executor<'_>> = OnceCell::new();

    fn global() -> &'static Executor<'static> {
        GLOBAL.get_or_init_blocking(|| Executor::new())
    }

    global().spawn(future)
}
