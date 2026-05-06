use crate::components::task::{Task, TaskKind};
use std::sync::{mpsc, Arc, Mutex};
use std::thread;
use std::time::Instant;
use std::sync::atomic::{AtomicBool, Ordering};


pub struct Dispatcher{
    pub cpu_queue: Arc<Mutex<Vec<Task>>>,
    pub io_queue: Arc<Mutex<Vec<Task>>>,
    pub done: Arc<AtomicBool>
}

impl Dispatcher{
    pub fn new() -> Self{
        Self{
            cpu_queue: Arc::new(Mutex::new(Vec::new())),
            io_queue: Arc::new(Mutex::new(Vec::new())),
            done: Arc::new(AtomicBool::new(false)),
        }
    }
pub fn start(&self, rx: mpsc::Receiver<Task>){
    let cpu_q = self.cpu_queue.clone();
    let io_q = self.io_queue.clone();

    let done_flag = self.done.clone();

    thread::spawn(move || {
        for mut task in rx{
            
            task.queued_time = Some(Instant::now());

            match task.kind{
                TaskKind::CPU => {
                    cpu_q.lock().unwrap().push(task);
                }
                TaskKind::IO => {
                    io_q.lock().unwrap().push(task);
                }
            }
        }
done_flag.store(true, Ordering::Relaxed);
        println!("Dispatcher: no more incoming tasks.");
    });
}
 
}
