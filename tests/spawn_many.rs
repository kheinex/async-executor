use async_executor::{Executor, LocalExecutor};
use futures_lite::future;

#[test]
fn spawn_many() {
    future::block_on(async {
        let ex = Executor::new();

        // Spawn a lot of tasks.
        let mut tasks = vec![];
        ex.spawn_many((0..50_000).map(|i| future::ready(i)), &mut tasks);

        // Run all of the tasks in parallel.
        ex.run(async move {
            for (i, task) in tasks.into_iter().enumerate() {
                assert_eq!(task.await, i);
            }
        })
        .await;
    });
}

#[test]
fn spawn_many_local() {
    future::block_on(async {
        let ex = LocalExecutor::new();

        // Spawn a lot of tasks.
        let mut tasks = vec![];
        ex.spawn_many((0..50_000).map(|i| future::ready(i)), &mut tasks);

        // Run all of the tasks in parallel.
        ex.run(async move {
            for (i, task) in tasks.into_iter().enumerate() {
                assert_eq!(task.await, i);
            }
        })
        .await;
    });
}
