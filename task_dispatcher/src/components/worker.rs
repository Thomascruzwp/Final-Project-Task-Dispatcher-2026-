use crate::components::dispatcher::Dispatcher;
use crate::components::metrics::Metrics;
use crate::components::task::TaskKind;

use std::sync::{Arc, Mutex};
use std::thread; 
use std::time::{Duration, Instant};
use std::sync::atomic::Ordering;

pub fn start_workers(
    dispatcher: Arc<Dispatcher>,
    metrics: Arc<Mutex<Metrics>>,
    count: usize,
){
    for _ in 0..count {
        let d = dispatcher.clone();
        let m = metrics.clone();

        thread::spawn(move || { 
            let mut local_busy = Duration::default();

            loop{
            let task_opt = {
                let mut cpu = d.cpu_queue.lock().unwrap();
                if let Some(t) = cpu.pop(){
                    Some(t)
                }else{
                    drop(cpu);
                    let mut io = d.io_queue.lock().unwrap();
                    io.pop()
                }
            };
            match task_opt{
                Some(mut t) => {
                    let start = Instant::now();
                    t.start_time = Some(start);

                    thread::sleep(Duration::from_millis(t.duration_ms));
                    let finish = Instant::now();
                    t.finish_time = Some(finish);
                    let exec_time = finish.duration_since(start);
                    let wait = start.duration_since(t.arrival_time);
                    let turnaround = finish.duration_since(t.arrival_time);

                    local_busy += finish.duration_since(start);

                    let mut met = m.lock().unwrap();

                    met.completed += 1;
                    met.total_busy += exec_time;
                    met.total_wait += wait;
                    met.total_turnaround += turnaround;

                    match t.kind{
                        TaskKind::CPU => met.cpu += 1, 
                        TaskKind::IO => met.io += 1,
                    }
                  
                    met.max_wait = met.max_wait.max(wait);
met.makespan = match met.makespan{
    Some(old) => Some(old.max(turnaround)),
    None => Some(turnaround),
                    };
                }

                None =>{

                    let (cpu_empty, io_empty)={
                        let cpu = d.cpu_queue.lock().unwrap();
                        let io = d.io_queue.lock().unwrap();
                        (cpu.is_empty(), io.is_empty())
                    };
                    
if d.done.load(Ordering::Relaxed) && cpu_empty && io_empty{
    break;
}
                    thread::sleep(Duration::from_millis(20));
                }
            }
        }

        });
    }
}



