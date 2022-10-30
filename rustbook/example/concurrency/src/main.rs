use std::sync::{ Arc, mpsc, Mutex }; 
use std::thread;
use std::time::Duration;

fn main() {
    //new_thread();    
    //move_value();
    //send_hi_from_thread();
    send_multiple_from_thread();
}

// Creating a new thread to print one thing while the main thread
// prints something else.
fn new_thread() {
    let handle = thread::spawn(|| {
        for i in 1..10 {
            println!("hi number {} from the spawned thread!", i);
            thread::sleep(Duration::from_millis(1));
        }
    });

    for i in 1..5 {
        println!("hi number {} from the main thread!", i);
        thread::sleep(Duration::from_millis(1));
    }
    
    // Saving a JoinHandle from thread::spawn to guarantee the thread is run
    // to completion
    handle.join().unwrap();
}

// Using the move keyword to force a closure to take ownership of the values 
// it uses
fn move_value(){
    let v = vec![1, 2, 3];

    let handle = thread::spawn(move || {
        println!("Here's a vector: {:?}", v);
    });

    handle.join().unwrap();
} 

// Moving tx to a spawned thread and sending "hi"
fn send_hi_from_thread() {
    let (tx,rx) = mpsc::channel();

    thread::spawn(move || {
        let val = String::from("hi");
        tx.send(val).unwrap();
    });

    // handle error properly in a real application instead of unwrapping
    let received = rx.recv().unwrap();
    println!("Got: {}", received);
}

// Sending multiple messages and pausing between each
fn send_multiple_from_thread() {
    let (tx, rx) = mpsc::channel();

    let tx1 = tx.clone();
    thread::spawn(move || {
        let vals = vec![
            String::from("hi"),
            String::from("from"),
            String::from("the"),
            String::from("tread"),
        ];

        for val in vals {
            tx1.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    thread::spawn(move || {
        let vals = vec![
            String::from("more"),
            String::from("messages"),
            String::from("for"),
            String::from("you"),
        ];

        for val in vals {
            tx.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    for received in rx {
        println!("Got: {}", received);
    }
}

// Exploring the API of Mutex<T> in a single-threaded context for simplicity
fn mutex_single_threaded() {
    let m = Mutex::new(5);

    {
        let mut num = m.lock().unwrap();
        *num = 6;
    }

    println!("m = {:?}", m);
}

// Ten threads each increment a counter guarded by a Mutex<T>
fn mutex_shared_value() {
    let counter = Arc::new(Mutex::new(0));
    let mut handles = vec![];

    for _ in 0..10 {
        let counter = Arc::clone(&counter);
        let handle = thread::spawn(move || {
            let mut num = counter.lock().unwrap();

            *num += 1;
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!("Result: {}", *counter.lock().unwrap());
}
