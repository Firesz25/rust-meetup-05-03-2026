use std::{
    pin::Pin,
    sync::{
        Arc, Mutex,
        mpsc::{Receiver, Sender, channel},
    },
    task::{Context, Poll},
    thread,
    time::{Duration, Instant},
};

use futures::task::{ArcWake, waker};

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
            println!("waiting for: {}", self.how_long);
            Poll::Ready(())
        } else {
            let waker = cx.waker().clone();
            let when = self.when;
            thread::spawn(move || {
                let now = Instant::now();
                if now < when {
                    thread::sleep(when - now);
                }
                waker.wake();
            });
            Poll::Pending
        }
    }
}

pub struct TaskFutures {
    future: Pin<Box<dyn Future<Output = ()> + Send>>,
    pull: Poll<()>,
}

impl TaskFutures {
    pub fn new<F: Future<Output = ()> + Send + 'static>(future: F) -> Self {
        Self {
            future: Box::pin(future),
            pull: Poll::Pending,
        }
    }

    pub fn pull(&mut self, cx: &mut Context) {
        if self.pull.is_pending() {
            self.pull = self.future.as_mut().poll(cx)
        }
    }
}

pub struct Task {
    // mutex only to implement Sync for Task futures
    future: Mutex<TaskFutures>,
    ex: Sender<Arc<Task>>,
}

impl Task {
    pub fn send(self: &Arc<Self>) {
        self.ex.send(self.clone()).unwrap();
    }

    pub fn new(future: TaskFutures, sender: Sender<Arc<Task>>) -> Self {
        Self {
            future: Mutex::new(future),
            ex: sender,
        }
    }

    pub fn spawn<F: Future<Output = ()> + Send + 'static>(future: F, sender: Sender<Arc<Task>>) {
        let task = Arc::new(Task::new(TaskFutures::new(future), sender.clone()));
        sender.send(task).unwrap();
    }

    pub fn pull(self: Arc<Self>) {
        let waker = waker(self.clone());
        let mut cx = Context::from_waker(&waker);

        self.future.try_lock().unwrap().pull(&mut cx);
    }
}

impl ArcWake for Task {
    fn wake_by_ref(arc_self: &Arc<Self>) {
        arc_self.send()
    }
}

pub struct SimpleExecutor {
    sender: Sender<Arc<Task>>,
    receiver: Receiver<Arc<Task>>,
}

impl SimpleExecutor {
    pub fn new() -> Self {
        let (tx, rx) = channel();
        Self {
            sender: tx,
            receiver: rx,
        }
    }

    pub fn spawn<F: Future<Output = ()> + Send + 'static>(&self, future: F) {
        Task::spawn(future, self.sender.clone());
    }

    pub fn run(&mut self) {
        while let Ok(task) = self.receiver.recv() {
            task.pull();
        }
    }
}

fn main() {
    let mut ex = SimpleExecutor::new();
    ex.spawn(Delay::new(Duration::from_secs(5)));
    ex.run();
}
