mod model;
mod api;
mod task;

use std::time::Instant;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let start_time = Instant::now();
    let tasks = task::create_fetch_tasks();
    let results = task::await_tasks(tasks).await;

    for (_, pokemon) in &results {
        println!("{:?}", pokemon);
    }

    let elapsed_time = start_time.elapsed();
    println!("Elapsed time: {:.2?}", elapsed_time);

    Ok(())
}
