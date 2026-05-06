# Final-Project-Task-Dispatcher-2026-
## Concurrent Task Dispatcher in Rust 

# Course Final Project 

## Concurrent Task Dispatcher Simulation (Rust)

---

# Overview 
This project is a concurrent task dispatcher system written in Rust that simulates an operating system style scheduler.

It models how tasks arrive over time, are placed into queues, and are executed by a bounded worker pool according to a scheduling policy.
 
* Concurrency(threads, shared state, synchronization)
 * Queue based scheduling systems
 * CPU vs IO workload behavior 
 * Performance measurement and system trade offs
 * Clean shutdown in multi-threaded Systems 
---

---
 # System Design Summary 
 the system consists of four main councurrent components:
--- 


 ## 1. Task Generator 
 * Creates tasks with random CPU/IO types 
 * Simulates arrival over time (delayed generation)
 * Sends tasks through a channel to the dispatcher 
 ---

 ## 2. Dispatcher 
 * Receives tasks from generator via channel 
 * Routes tasks into: 
    * CPU queue 
    * IO queue 
* Marks when no more task will arrive 
---

## 3. Worker Pool 
* Fixed size pool (8 workers)
* Continuously pull tasks from queues 
* Executes tasks using simulated sleep 
* Records execution statistics 
---

## 4. Monitor 
* Observes system progress in real time 
* Prints live updates of completed tasks 
---


# Task Model 
Each task includes: 
* 'id' -> unqiue identifier 
* 'arrival_time' -> when task is created 
* 'queued_time' -> when added to queue 
* 'start_time' -> execution start 
* 'finish_time' -> execution end 
* 'kind' -> CPU or IO 
* 'duration_ms' -> simulated runtime 
--- 

# Scheduling Policy

## Policy Used: CPU- Priority Two-Queue Scheduling 
* CPU tasks are prioritized over IO tasks
* Workers always attempt CPU queue first 
* if CPU queue is empty, IO queue is used 
---

## Why this policy? 
This policy simulates systems where CPU bound work is more critical and should be prioritized for throughput. 
---

## Trade-offs 
### Advantages: 
* High CPU utilization 
* Simple decision making logic 
* Predictable scheduling behavior 

### Disadvantages: 
* IO tasks may experience longer waiting times 
* Potential starvation under CPU- heavy workloads
---

# Concurrency Model 
The system uses multiple threads:
* Generator thread -> produces tasks 
* Dispacther thread -> routes tasks into queues 
* Worker threads (8) -> execute tasks concurrently 
* Monitor thread -> prints live metrics 
--- 

## Shared state is protected using: 
* Arc<Mutex<...>> for queues and metrics 
* AtomicBool for shutdown signaling 
* Channels for safe task transfer 
---

# Queue-Based Architecture 
Two main queues are used: 
* CPU queue 
* IO queue 
--- 

Workers pull tasks from queues using priority rules. 
This design allows workload separation and scheduling control 
---

# Metrics Collected 

## Required Metrics 
* Total tasks completed 
* Makespan 
* Average wait time 
* Average turnaround time 
--- 

## Additional Metrics 
* CPU vs IO task counts 
* Worker utilization 
* Maximum wait time 
---


# Experiments 
---

## Experiment A: Balanced Workload 
* 500 tasks 
* Mixed CPU/IO distribution 

## Results: 
* High utilization (~99%)
* Balanced completion between CPU and IO tasks 
* Moderate wait turnaround times 
--- 

## Experiment B: Stress Workload 
* 1000 taks 
* Heavier system load 

Results: 
* Worker utilization reaches ~ 100% 
* increased wait and turnaround time 
* System remains stable under pressure 
---


## How to Build and Run 
In the terminal run these commands 
---

Build 
Cargo build 

Run 
Cargo run 
--- 
## Debug tip 

if you unable to cargo build, first check your project location and files 
--- 

1. Check current directory 
ls 
this should show a cargo.toml file and sec/ folder 

2. Move into the correct folder 
if you are not inside the project folder, use: 

cd your_project_name 

then try again: 

Cargo build 

3. Common issues 
* make sure Rust and Cargo are installed (rustc --version)
* Make sure you are inside the folder that contains Cargo.toml
* Do not run cargo build outside the project directory 


--- 
# Example Output 
== Balanced == 
[Monitor] completed =120 cpu=35 io=85 

RESULT [Balanced]
completed = 500
cpu = 147 
io = 353 
avg_wait = 1.21s 
avg_turnaround = 1.41s 
makespan = 12.53s 
worker utilization = 99.83% 
--- 

## Clean Shutdown Behavior 
The systems shuts down when: 
* All tasks have been generated 
* Dispatcher has finished routing tasks
* Worker queues are empty 
* All workers complete execution 
--- 

This prevents infinite loops and ensures graceful termination 
---

# Know Limitations 
* Workers use polling instead of fully blocking queues 
* Monitor thread does not explicity join on shutdown 
* Scheduling policy is static (no dynamic adaptation)
* IO starvation is possbile under CPU-heavy workloads 
--- 


# Possible Improvements 
* Work stealing worker design 
* Round robin or fair scheduling 
* Aging to reduce starvation 
* Priority based tasks 
* Backpressure (bounded queues)
* Explicit thread join-based shutdown system 
---

# Key Concepts Demonstrated 
* Rust concurrency (threads, Arc, Mutex)
* Producer- consumer architecure 
* Queue-based scheduling 
* CPU vs IO workload modeling 
* Performance measurement and analysis 
* System shutdown coordination 

---

## Tool Use Disclosure 
During the development of this project, I used external resources to support learning and understanding of concurrency concepts and Rust implementation details.

# Tools and resources used 
* Rust Compiler (rustc)
* Cargo (Rust build system and package manager)
* Rust Standard Library Documentation 
* Course lecture materials and notes 
* GitHub (version control and repository hosting)

# How it was used 
Understanding and applying concurrency concept such as thread, Arc, Mutex, and channels (mpsc)
Organizing the project structure and separating system components (generator, dispatcher, workers, monitor).

Managing version control, tracking changes, and storing project iterations using Github Supporting the explanation and implementation of scheduling policies and metrics calculation assistign with debugging and reasoning about multi threaded execution flow.

# Example of advice I accepted 
Using a two queue system (CPU queue and IO queue) to separate workloads 

structuring the system into clear components: generator, dispatcher, worker, pool, and monitor tracking full task lifecycle (arrival -> queue -> execution -> completion) for accurate metrics.

# Example of advice I modified or rejected 
Rejected a single shared queue design because it reduced clarity between CPU and IO scheduling behavior modified the idea of fully blocking queues and kept a polling based worker approach for simpler control and shutdown handling. 

Did not simplify metrics collection because full lifecycle tracking was required for meaningful performance analysis and comparison between experiments.

## Author Notes 
This project demonstrates a a simplified operating system scheduler simulation.

The focus is on understanding how 
concurrency. Scheduling polices, and shared state interact in a controlled system.
---