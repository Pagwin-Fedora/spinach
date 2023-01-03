use crate::thread::*;
use crate::worker::*;

#[non_exhaustive]
pub struct Runtime{
    threads: Vec<SpinThread>,
    workers: Vec<Worker>
}

impl Runtime {
    fn lazy_new()->Self{
        Runtime{
            threads:Vec::new(),
            workers:Vec::new()
        }
    }
    fn new(worker_count:usize)->Self{
        let mut workers = Vec::new();
        let (send,mut recv) = std::sync::mpsc::channel::<std::sync::Arc<Worker>>();
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
}
impl Default for Runtime {
    fn default()->Self{
        Self::lazy_new()
    }
}
