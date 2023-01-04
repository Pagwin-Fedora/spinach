use crate::thread::*;
use std::sync::mpsc::Sender;
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
    os_thread:std::thread::Thread,
    handle: WorkerHandle,
    queue:std::sync::mpsc::Receiver<Task>,
    status:WorkerStatus
}
impl Worker {
    pub fn new_laborer(channel:Sender<WorkerHandle>){
        std::thread::spawn(||{Worker::create_self(channel)});
    }
    fn create_self(channel:Sender<WorkerHandle>){
        let os_thread = std::thread::current();
        let (handle, queue) = WorkerHandle::gen_handle();
        let status = WorkerStatus::Idling;
        let spawned_worker = Worker {
            os_thread,
            handle,
            queue,
            status
        };

        channel.send(spawned_worker.get_handle()).expect("Sending of worker handle failed");
        spawned_worker.exec_loop();
    }
    fn exec_loop(mut self){
        loop{
            self.status = WorkerStatus::Idling;
            match self.queue.recv() {
                Ok(msg)=>{
                    if let Task::Die = msg {
                        break
                    }
                    self.process_msg(msg);
                },
                Err(_)=>{
                    // if we get an error when receiving a msg we die to avoid wasting resources
                    break
                }
           }
        }
    }
    fn process_msg(&mut self, msg:Task){
        
    }
    pub fn get_handle(&self)->WorkerHandle{
        self.handle.clone()
    }
}

#[derive(Clone)]
pub struct WorkerHandle{
    channel:std::sync::mpsc::Sender<Task>
}
impl WorkerHandle {
    fn gen_handle()->(WorkerHandle,std::sync::mpsc::Receiver<Task>) {
        let (send,recv) = std::sync::mpsc::channel();
        (WorkerHandle{channel:send},recv)
    }
}
