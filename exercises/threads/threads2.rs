// threads2.rs
// Execute `rustlings hint threads2` or use the `hint` watch subcommand for a hint.
// Building on the last exercise, we want all of the threads to complete their work but this time
// the spawned threads need to be in charge of updating a shared value: JobStatus.jobs_completed

use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

struct JobStatus {
    jobs_completed: u32,
}

fn main() {
    let status = Arc::new(Mutex::new(JobStatus { jobs_completed: 0 }));
    let mut handles = vec![];
    for _ in 0..10 {
        let status_shared = status.clone();
        let handle = thread::spawn(move || {
            thread::sleep(Duration::from_millis(1000));
            status_shared.lock().unwrap().jobs_completed += 1;
        });
        handles.push(handle);
    }

    while status.lock().unwrap().jobs_completed < 10 {
        thread::sleep(Duration::from_millis(200));
    }

    println!("jobs completed: {}", status.lock().unwrap().jobs_completed);
}
