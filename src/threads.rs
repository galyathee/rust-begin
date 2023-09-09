use std::thread;

pub fn main() {
    // Create a new thread and spawn it
    let thread1 = thread::spawn(|| {
        for i in 1..=5 {
            println!("Thread 1: Count {}", i);
        }
    });

    // Create another thread and spawn it
    let thread2 = thread::spawn(|| {
        for i in 1..=5 {
            println!("Thread 2: Count {}", i);
        }
    });

    // Wait for both threads to finish using the `join` method
    thread1.join().unwrap();
    thread2.join().unwrap();

    // Main thread continues here
    println!("Main thread: All threads have finished.");
}
