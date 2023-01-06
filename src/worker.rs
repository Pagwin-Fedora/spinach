use crate::thread::*;
use std::sync::mpsc;
enum WorkerStatus{
    Executing(Task),
    Idling
}
pub enum Task {
    Thread(SpinThread),
    Die,
    Manage(Vec<WorkerHandle>),
    SpawnPool(usize),
    GiveInfo(mpsc::Sender<InfoResponse>, InfoRequest)
}
type ID = usize;
/// A message requesting some info that should be responded to with the corresponding [InfoResponse] the ID sent with the message will be sent back with the response to allow for identification
pub enum InfoRequest {
    QueueSize(ID)
}
pub enum InfoResponse {
    QueueSize(ID, usize)
}
pub struct Worker{
    os_thread:std::thread::Thread,
    handle: WorkerHandle,
    recv:mpsc::Receiver<Task>,
    queue: std::collections::VecDeque<Task>,
    status:WorkerStatus
}
impl Worker {
    pub fn new_laborer(channel:mpsc::Sender<WorkerHandle>){
        std::thread::spawn(||{Worker::create_self(channel)});
    }
    fn create_self(channel:mpsc::Sender<WorkerHandle>){
        let os_thread = std::thread::current();
        let (handle, recv) = WorkerHandle::gen_handle();
        let status = WorkerStatus::Idling;
        let queue = std::collections::VecDeque::new();
        let spawned_worker = Worker {
            os_thread,
            handle,
            recv,
            queue,
            status,
        };

        channel.send(spawned_worker.get_handle()).expect("Sending of worker handle failed");
        spawned_worker.exec_loop();
    }
    fn exec_loop(mut self){
        loop{
            while let Ok(task) = self.recv.try_recv() {
                self.queue.push_back(task);
            }

            match self.queue.pop_front() {
                Some(msg)=>{
                    if let Task::Die = msg {
                        break
                    }
                    self.process_msg(msg);
                },
                None=>{
                    // if we don't have any messages in queue we should wait a short time before
                    // seeing if we got any more, 5 is an arbitrary wait time which should be
                    // decided in a smarter way in future TODO: make the sleep duration smarter
                    std::thread::sleep(std::time::Duration::from_millis(5));
                }
           }
        }
    }
    fn process_msg(&mut self, msg:Task){
        match msg {
            Task::Thread(_thread)=>{
                match self.status {
                    WorkerStatus::Idling => {
                        // thread.resume();

                    },
                    WorkerStatus::Executing(task)=>{
                        if let Task::Manage(pool) = task{
                            
                        }
                        else {
                            panic!("We read a new message when we were busy with something other than managing a pool");
                        }
                    }
                }
            },
            Task::Manage(workers)=>{
                
            },
            Task::SpawnPool(count) => {
                let pool = Worker::spawn_laborers(count);
                if pool.iter().all(Result::is_ok){
                    // unsafe block is so we can unwrap unchecked which is fine because we just
                    // checked all the results are Ok
                    unsafe {
                        // can't map unsafe functions D:
                        let mut managed_pool = Vec::with_capacity(pool.len());
                        for worker in pool.into_iter() {
                            managed_pool.push(worker.unwrap_unchecked());
                        }
                        self.status = WorkerStatus::Executing(Task::Manage(managed_pool));
                    }
                }
                else {

                }
            }
            Task::Die => panic!("Death request got further than it should've"),
            Task::GiveInfo(send,request)=>{
                match request {
                    InfoRequest::QueueSize(id)=> send.send(InfoResponse::QueueSize(id,self.queue.len()))
                };
            }
        }
    }
    pub fn spawn_laborers(amount:usize)->Vec<Result<WorkerHandle, LaborSpawnError>>{
        let mut ret = Vec::new();
        let (send,recv) = mpsc::channel();
        for _ in 0..amount {
            Worker::new_laborer(send.clone());
        }
        for _ in 0..amount {
            ret.push(recv.recv().map_err(LaborSpawnError::from))
        }
        ret
    }
    pub fn get_handle(&self)->WorkerHandle{
        self.handle.clone()
    }
}

pub enum LaborSpawnError{ChannelRecvError(mpsc::RecvError)}

impl From<mpsc::RecvError> for LaborSpawnError{
    fn from(err:mpsc::RecvError)->Self{
        Self::ChannelRecvError(err)
    }
}

#[derive(Clone)]
pub struct WorkerHandle{
    pub channel:mpsc::Sender<Task>
}
impl WorkerHandle {
    fn gen_handle()->(WorkerHandle,mpsc::Receiver<Task>) {
        let (send,recv) = mpsc::channel();
        (WorkerHandle{channel:send},recv)
    }
}
