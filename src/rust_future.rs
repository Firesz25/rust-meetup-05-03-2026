use std::time::Duration;

pub async fn do_something1() -> Result<(), ()> {
    tokio::time::sleep(Duration::from_secs(10)).await;
    Ok(())
}

pub fn do_something2() -> impl Future<Output = Result<(), ()>> {
    async {
        tokio::time::sleep(Duration::from_secs(10)).await;
        Ok(())
    }
}
