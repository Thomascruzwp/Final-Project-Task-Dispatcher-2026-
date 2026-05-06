mod components;

use components::dispatcher::Dispatcher;
use components::metrics::Metrics;
use components::experiments::{exp_a, exp_b};
use components::{worker, monitor, generator};

use std::sync::{mpsc, Arc, Mutex};
use std::thread;
use std::time::Duration;

fn run(name: &str, tasks: usize){
    println!("\n== {} ===", name);
let (tx, rx) = mpsc::channel();

let dispatcher = Arc::new(Dispatcher::new());
let metrics = Arc::new(Mutex::new(Metrics::default()));

dispatcher.start(rx);

worker::start_workers(dispatcher.clone(), metrics.clone(), 8);
monitor::start_monitor(metrics.clone());
generator::start_generator(tx, tasks);

loop{
        let m = metrics.lock().unwrap();
        if m.completed >= tasks{
            break;
        }
drop(m);
    thread::sleep(Duration::from_millis(100));
}

let m = metrics.lock().unwrap();

let completed = m.completed as u32;

let avg_wait = if completed > 0{
    m.total_wait / completed
}else{
    Duration::from_secs(0)
};

let avg_turnaround = if completed > 0{
    m.total_turnaround / completed
}else{
    Duration::from_secs(0)
};

let makespan = m.makespan.unwrap_or(Duration::from_secs(0));



let makspan_secs = makespan.as_secs_f64();
let total_busy_time = m.total_busy.as_secs_f64();
let total_capacity = 8.0 * makspan_secs;

let utilization = if total_capacity > 0.0 {
    (total_busy_time / total_capacity) * 100.0
}else{
    0.0
};




println!("\nRESULT [{}]", name);
println!("completed = {}", m.completed);
println!("cpu = {}", m.cpu);
println!("io = {}", m.io);
println!("avg_turnaround = {:?}", avg_turnaround);
println!("avg_wait = {:?}", avg_wait);
println!("makespan = {:?}", m.makespan.unwrap());

println!("worker utilization = {:.2}%", utilization);
}

fn main(){
    let a = exp_a();
    let b = exp_b();

    run(a.name, a.tasks);
    run(b.name, b.tasks);
}