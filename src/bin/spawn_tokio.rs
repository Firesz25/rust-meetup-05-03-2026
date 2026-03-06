use tokio::time::{Duration, sleep};

#[tokio::main]
async fn main() {
    // 1. Spawning a background task
    // This moves to the background immediately
    let handle = tokio::spawn(async {
        println!("Background task started...");
        sleep(Duration::from_secs(2)).await;
        "Task Finished!" // Return value
    });

    println!("The main function is NOT blocked and continues...");

    // 2. Doing other work while the task runs
    sleep(Duration::from_secs(1)).await;
    println!("Main is doing other things...");

    // 3. Waiting for the spawned task result (JoinHandle)
    match handle.await {
        Ok(result) => println!("Result: {}", result),
        Err(e) => eprintln!("Task panicked: {}", e),
    }
}
