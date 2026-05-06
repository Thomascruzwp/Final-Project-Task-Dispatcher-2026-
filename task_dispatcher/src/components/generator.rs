use crate::components::task::{Task, TaskKind};
use rand::Rng;
use::std::sync::mpsc;
use std::thread;
use std::time::{Duration, Instant};

pub fn start_generator(tx: mpsc::Sender<Task>, tasks: usize){
    thread::spawn(move || {
        let mut rng = rand::thread_rng();

        for id in 0..tasks {
            let kind = if rng.gen_range(0..100) < 70{
                TaskKind::IO
            }else{
                TaskKind::CPU
            };

            let task = Task {
                id, 
                arrival_time: Instant::now(),
                queued_time: Some(Instant::now()),
                
                start_time: None,
                finish_time: None,
                kind,
                duration_ms: 200,
            };

            tx.send(task).unwrap();
            thread::sleep(Duration::from_millis(20));
        }
    });
}