use crossbeam_channel::{unbounded, Receiver, Sender};
use std::sync::{Arc, Mutex};
use std::thread::{spawn, JoinHandle};

type Job = Box<dyn FnOnce() + Send>;

pub struct ThreadPool {
    workers: Vec<Worker>,
    sender: Sender<Message>
}

impl ThreadPool {
    pub fn new(size: usize) -> Self {
        let (tx, rx) = unbounded::<Message>();
        let rx = Arc::new(Mutex::new(rx));
        let workers = (0..size).map(|_| Worker::new(rx.clone())).collect();

        Self {
            workers,
            sender: tx,
        }
    }

    pub fn execute<F: FnOnce() + Send + 'static>(&self, callback: F) {
        let message = Message::Job(Box::new(callback));
        self.sender.send(message).unwrap();
    }
}

impl Drop for ThreadPool {
    fn drop(&mut self) {
        self.sender.send(Message::Stop).unwrap();
    }
}

enum Message {
    Job(Job),
    Stop,
}

struct Worker {
    thread: Option<JoinHandle<()>>
}

impl Worker {
    pub fn new(receiver: Arc<Mutex<Receiver<Message>>>) -> Self {
        let thread = spawn(move || loop {
            let job = receiver.lock().unwrap().recv().unwrap();
            match job {
                Message::Job(job) => (job)(),
                Message::Stop => break,
            };
        });

        Self {
            thread: Some(thread),
        }
    }
}

impl Drop for Worker {
    fn drop(&mut self) {
        if let Some(thread) = self.thread.take() {
            thread.join().unwrap();
        }
    }
}
