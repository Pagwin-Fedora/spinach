use crate::thread::*;

enum WorkerStatus{
    Running(SpinThread),
    Idling
}
pub struct Worker{
    handle:std::thread::Thread,
    queue: std::collections::VecDeque<SpinThread>,
    status:WorkerStatus
}
