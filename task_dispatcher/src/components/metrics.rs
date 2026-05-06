use std::time::Duration;

#[derive(Default)]
pub struct Metrics{
    pub completed: usize,
    pub cpu: usize,
    pub io: usize,

    pub total_wait: Duration,
    pub total_turnaround: Duration,

    pub max_wait: Duration,
    pub makespan: Option<Duration>, 

    pub total_busy: Duration
}