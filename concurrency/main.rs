use std::thread;

fn main() {
    // Spawn two threads
    let handle1 = thread::spawn(|| {
        for i in 1..5 {
            println!("Thread 1: Count {}", i);
            thread::sleep(std::time::Duration::from_millis(500));
        }
    });

    let handle2 = thread::spawn(|| {
        for i in 1..5 {
            println!("Thread 2: Count {}", i);
            thread::sleep(std::time::Duration::from_millis(700));
        }
    });

    // Wait for both threads to finish
    handle1.join().unwrap();
    handle2.join().unwrap();

    println!("Both threads have finished.");
}
