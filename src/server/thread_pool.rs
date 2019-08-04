use std::thread::{spawn, JoinHandle};
use crossbeam_channel::{unbounded, Sender, Receiver};
use std::sync::{Arc, Mutex};

type Job = Box<dyn FnOnce() + Send>;

pub struct ThreadPool {
    workers: Vec<Worker>,
    sender: Sender<Message>
}

impl ThreadPool {
    pub fn new(size: usize) -> Self {
        let (tx, rx) = unbounded::<Message>();
        let rx = Arc::new(Mutex::new(rx));
        let workers = (0..size)
            .map(|i| Worker::new(i, rx.clone()))
            .collect();

        Self {
            workers,
            sender: tx
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
    Stop
}

struct Worker {
    id: usize,
    thread: Option<JoinHandle<()>>
}

impl Worker {
    pub fn new(id: usize, receiver: Arc<Mutex<Receiver<Message>>>) -> Self {
        let thread = spawn(move || {
            loop {
                match receiver.lock().unwrap().recv().unwrap() {
                    Message::Job(job) => (job)(),
                    Message::Stop => break
                };
            }
        });

        Self {
            id,
            thread: Some(thread)
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