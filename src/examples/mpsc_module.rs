use std::sync::mpsc;
use std::thread;
use std::time::Duration;

fn main() {
    let (tx, rx) = mpsc::channel();
    println!("Hello from main thread");

    let tx1 = tx.clone();
    let tx2 = tx.clone();

    // Move keyword moves the ownership of captured variables to closure scope
    // Moving is necessary in threads
    // Only variables that are used by the closure are moved
    let h1 = thread::spawn(move || {
        thread::sleep(Duration::from_millis(5000));
        let s = "Hello from thread 1";
        tx1.send(s).unwrap();
    });

    let h2 = thread::spawn(move || {
        thread::sleep(Duration::from_millis(5000));
        let s = "Hello from thread 2";
        tx2.send(s).unwrap();
    });

    // It is not possible to determine which thread will send the message first
    // Main thread halts here until a message is received from the channel
    println!("Waiting for message...");
    let s = rx.recv().unwrap();
    println!("Received message: {}", s);

    // Main thread halts here again
    // If no message is received from the channel, the main thread will wait indefinitely
    println!("Waiting for message...");
    let s = rx.recv().unwrap();
    println!("Received message: {}", s);

    // Threads are joined to main thread to ensure they complete before the program exits
    h1.join().unwrap();
    h2.join().unwrap();
}
