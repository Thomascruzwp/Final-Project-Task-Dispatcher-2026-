use crate::components::metrics::Metrics;
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

pub fn start_monitor(metrics: Arc<Mutex<Metrics>>){
    thread::spawn(move || {
        let mut last = (0, 0, 0);
        loop{
            let snapshot ={
                let m = metrics.lock().unwrap();
                (m.completed, m.cpu, m.io)
            };

          
if snapshot != last{
                println!(
                "[MONITOR] completed={} cpu={} io={}",
                snapshot.0, snapshot.1, snapshot.2,
            );

            last = snapshot;
        }
            
             thread::sleep(Duration::from_millis(100)); 
        }
    });
}
