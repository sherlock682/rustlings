// threads3.rs
// Execute `rustlings hint threads3` or use the `hint` watch subcommand for a hint.

use std::sync::{mpsc, Arc, Mutex};
use std::thread;
use std::time::Duration;

struct Queue {
    length: u32,
    first_half: Vec<u32>,
    second_half: Vec<u32>,
}

impl Queue {
    fn new() -> Self {
        Queue {
            length: 10,
            first_half: vec![1, 2, 3, 4, 5],
            second_half: vec![6, 7, 8, 9, 10],
        }
    }
}

fn send_tx(q: Arc<Mutex<Queue>>, tx: Arc<Mutex<mpsc::Sender<u32>>>) {
    let q1 = Arc::clone(&q);
    let q2 = Arc::clone(&q);
    let tx1 = Arc::clone(&tx);
    let tx2 = Arc::clone(&tx);

    let thread1 = thread::spawn(move || {
        let queue = q1.lock().unwrap();
        for val in &queue.first_half {
            println!("sending {:?}", val);
            let sender = tx1.lock().unwrap();
            sender.send(*val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    let thread2 = thread::spawn(move || {
        let queue = q2.lock().unwrap();
        for val in &queue.second_half {
            println!("sending {:?}", val);
            let sender = tx2.lock().unwrap();
            sender.send(*val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    thread1.join().unwrap();
    thread2.join().unwrap();
}

fn main() {
    let (tx, rx) = mpsc::channel();
    let queue = Arc::new(Mutex::new(Queue::new()));
    let queue_length = queue.lock().unwrap().length;

    let tx = Arc::new(Mutex::new(tx));
    send_tx(Arc::clone(&queue), Arc::clone(&tx));

    let mut total_received: u32 = 0;
    for _ in 0..queue_length {
        if let Ok(received) = rx.recv() {
            println!("Got: {}", received);
            total_received += 1;
        }
    }

    println!("total numbers received: {}", total_received);
    assert_eq!(total_received, queue_length);
}

