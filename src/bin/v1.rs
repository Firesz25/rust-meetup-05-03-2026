use std::{
    pin::Pin,
    task::{Context, Poll},
    time::{Duration, Instant},
};

pub struct Delay {
    when: Instant,
    /// use only for returning hao long is delay in second
    how_long: u64,
}

impl Delay {
    pub fn new(dur: Duration) -> Self {
        Self {
            when: Instant::now() + dur,
            how_long: dur.as_secs(),
        }
    }
}

impl Future for Delay {
    type Output = u64;

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        if Instant::now() >= self.when {
            println!("futures ready");
            Poll::Ready(self.how_long)
        } else {
            // ignore this for while
            cx.waker().wake_by_ref();
            Poll::Pending
        }
    }
}

#[tokio::main]
async fn main() {
    let future = Delay::new(Duration::from_secs(5));
    let del = future.await;
    assert_eq!(del, 5);
}
