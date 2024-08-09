// Building on the last exercise, we want all of the threads to complete their
// work. But this time, the spawned threads need to be in charge of updating a
// shared value: `JobStatus.jobs_done`

use anyhow::Result;
use std::{
    sync::{Arc, Mutex},
    thread,
    time::Duration,
};

struct JobStatus {
    jobs_done: u32,
}

fn main() -> Result<()> {
    // TODO: `Arc` isn't enough if you want a **mutable** shared state.
    let status = Arc::new(Mutex::new(JobStatus { jobs_done: 0 }));

    let mut handles = Vec::new();
    for _ in 0..10 {
        let status_shared = Arc::clone(&status);
        let handle = thread::spawn(move || -> Result<()> {
            thread::sleep(Duration::from_millis(250));

            // TODO: You must take an action before you update a shared value.
            let mut status = status_shared
                .lock()
                .map_err(|_| anyhow::anyhow!("Mutex poisoned"))?;
            status.jobs_done += 1;
            Ok(())
        });
        handles.push(handle);
    }

    // Waiting for all jobs to complete.
    for handle in handles {
        let _ = handle
            .join()
            .map_err(|_| anyhow::anyhow!("Thread panicked"))?;
    }

    // TODO: Print the value of `JobStatus.jobs_done`.
    println!(
        "Jobs done: {}",
        status
            .lock()
            .map_err(|_| anyhow::anyhow!("Mutex poisoned"))?
            .jobs_done
    );

    Ok(())
}
