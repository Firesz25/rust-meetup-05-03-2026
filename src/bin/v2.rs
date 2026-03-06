use futures::task::noop_waker;
use std::{
    collections::VecDeque,
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
    type Output = ();

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        if Instant::now() >= self.when {
            println!("futures ready");
            println!("waiting: {}", self.how_long);
            Poll::Ready(())
        } else {
            // ignore this for while
            cx.waker().wake_by_ref();
            Poll::Pending
        }
    }
}

type Task = Pin<Box<dyn Future<Output = ()>>>;

pub struct SimpleExecutor {
    task: VecDeque<Task>,
}

impl SimpleExecutor {
    pub fn new() -> Self {
        Self {
            task: VecDeque::new(),
        }
    }

    pub fn spawn<F: Future<Output = ()> + Send + 'static>(&mut self, task: F) {
        self.task.push_back(Box::pin(task));
    }

    pub fn run(&mut self) {
        let waker = noop_waker();
        let mut ctx = Context::from_waker(&waker);
        while let Some(mut future) = self.task.pop_front() {
            if future.as_mut().poll(&mut ctx).is_pending() {
                self.task.push_back(future);
            }
        }
    }
}

fn main() {
    let mut ex = SimpleExecutor::new();
    ex.spawn(Delay::new(Duration::from_secs(5)));
    ex.run();
}
