use std::sync::{Arc, Mutex};
use std::thread;

fn main() {
    // Shared counter with Mutex wrapped in Arc
    let counter = Arc::new(Mutex::new(0));
    let mut handles = vec![];

    for i in 0..5 {
        let counter = Arc::clone(&counter);
        let handle = thread::spawn(move || {
            for _ in 0..10 {
                let mut num = counter.lock().unwrap();
                *num += 1;
            }
            println!("Thread {} finished incrementing.", i + 1);
        });
        handles.push(handle);
    }

    // Wait for all threads to complete
    for handle in handles {
        handle.join().expect("Thread failed to join");
    }

    println!("Final counter value: {}", *counter.lock().unwrap());
}

