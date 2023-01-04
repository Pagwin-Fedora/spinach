use crate::thread::*;
use crate::worker::*;
use crate::state_management::*;

#[non_exhaustive]
pub struct Runtime{
    threads: Vec<SpinThread>,
    workers: Vec<WorkerHandle>
}

impl Runtime {
    fn lazy_new()->Self{
        Runtime{
            threads:Vec::new(),
            workers:Vec::new()
        }
    }
    pub fn new(worker_count:usize)->Self{
        let mut workers = Vec::new();
        let (send, recv) = std::sync::mpsc::channel();
        //multi threaded worker creation because might as well have workers bootstrap themselves
        for _ in 0..worker_count {
            Worker::new_laborer(send.clone());
        }
        for _ in 0..worker_count {
            workers.push(recv.recv()
                .expect("mpsc error when creating worker pool"));
        }
        Runtime {
            threads: Vec::new(),
            workers
        }
    }
    pub fn new_consume<T:ThreadState>(){

    }
}
impl Default for Runtime {
    fn default()->Self{
        Self::lazy_new()
    }
}
