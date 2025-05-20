use futures::join;
use gloo_timers::future::TimeoutFuture;
pub async fn generate_preview() {
    join!(
        async {
            TimeoutFuture::new(3000).await;
            eprintln!("Task 1 complete");
        },
        async {
            TimeoutFuture::new(1000).await;
            eprintln!("Task 2 complete");
        },
        async {
            TimeoutFuture::new(2000).await;
            eprintln!("Task 3 complete");
        }
    );
}
