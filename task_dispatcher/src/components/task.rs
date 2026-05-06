use std::time::Instant;

#[derive(Clone)]
pub enum TaskKind{
    CPU,
    IO,
}

#[derive(Clone)]
pub struct Task{
pub id: usize,
pub arrival_time: Instant,

pub queued_time: Option<Instant>,
pub start_time: Option<Instant>,
pub finish_time: Option<Instant>,

pub kind: TaskKind,
pub duration_ms: u64,

}