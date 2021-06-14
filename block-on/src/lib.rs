use waker_fn::waker_fn;      // Cargo.toml: waker-fn = "1.1"
use futures_lite::pin;       // Cargo.toml: futures-lite = "1.11"
use crossbeam::sync::Parker; // Cargo.toml: crossbeam = "0.8"
use std::future::Future;
use std::task::{Context, Poll};

pub fn block_on<F: Future>(future: F) -> F::Output {
    let parker = Parker::new();
    let unparker = parker.unparker().clone();
    let waker = waker_fn(move || unparker.unpark());
    let mut context = Context::from_waker(&waker);

    pin!(future);

    loop {
        match future.as_mut().poll(&mut context) {
            Poll::Ready(value) => return value,
            Poll::Pending => parker.park(),
        }
    }
}

#[test]
fn test() {
    assert_eq!(block_on(std::future::ready(42)), 42);

    use async_std::task::{spawn, sleep};
    use futures_lite::FutureExt;
    use std::time::Duration;

    assert_eq!(
        block_on({
            let one_sec = async {
                sleep(Duration::from_secs(1)).await;
                43
            };
            let half_sec = async {
                sleep(Duration::from_millis(500)).await;
                44
            };
            spawn(one_sec.race(half_sec))
        }),
        44);
}
