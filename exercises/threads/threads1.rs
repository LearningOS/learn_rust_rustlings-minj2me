// threads1.rs
// Make this compile! Execute `rustlings hint threads1` for hints :)
// The idea is the thread spawned on line 22 is completing jobs while the main thread is
// monitoring progress until 10 jobs are completed. Because of the difference between the
// spawned threads' sleep time, and the waiting threads sleep time, when you see 6 lines
// of "waiting..." and the program ends without timing out when running,
// you've got it :)

use std::ops::{Deref, DerefMut};
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

struct JobStatus {
    jobs_completed: u32,
}

fn main() {
    //let status = Arc::new(JobStatus { jobs_completed: 0 });
    /*
    Mutexes have a reputation for being difficult to use because you have to remember two rules:
    1. You must attempt to acquire the lock before using the data.
    2. When you’re done with the data that the mutex guards, you must unlock the data so other threads can acquire the lock.
    */
    let status = Arc::new(Mutex::new(JobStatus { jobs_completed: 0 }));
    let status_shared = status.clone();
    thread::spawn(move || {
        for _ in 0..10 {
            thread::sleep(Duration::from_millis(250));
            let mut job_status = status.lock().unwrap();
            job_status.jobs_completed+=1;
        }
    });
    while status_shared.lock().unwrap().jobs_completed < 10 {
        println!("waiting... ");
        thread::sleep(Duration::from_millis(500));
    }
}
