use crate::thread::*;
use std::sync::mpsc::Sender;
use std::sync::Arc;
enum WorkerStatus{
    Executing(Task),
    Idling
}
enum Task {
    Thread(SpinThread),
    Die,
    Manage(Vec<Worker>),
    SpawnPool
}
pub struct Worker{
    handle:std::thread::Thread,
    input: std::sync::mpsc::Sender<Task>,
    queue:std::sync::mpsc::Receiver<Task>,
    status:WorkerStatus
}
// nobody should be touching workers outside of sending them messages and the only issue is the
// Thread inside the worker, in the future this may be reworked but for now Send impl should be
// fine
unsafe impl Send for Worker{}
unsafe impl Send for Task{}
unsafe impl Send for WorkerStatus{}

impl Worker {
    pub fn new_laborer(channel:Sender<Arc<Worker>>){
        std::thread::spawn(||{Worker::create_self(channel)});
    }
    fn create_self(channel:Sender<Arc<Worker>>){
        
        let handle = std::thread::current();
        let (input, queue) = std::sync::mpsc::channel();
        let status = WorkerStatus::Idling;
        let spawned_worker = Worker {
            handle,
            input,
            queue,
            status
        };
        let ptr = Arc::from(spawned_worker);

        channel.send(Arc::clone(&ptr));
        ptr.exec_loop();
    }
    fn exec_loop(&mut self){

    }
}
