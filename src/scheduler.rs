use std::sync::mpsc;
use crate::worker::{Task,WorkerHandle, InfoRequest, InfoResponse};

pub trait Scheduler{
    fn schedule(&mut self, task:Task, laborers:Vec<WorkerHandle>);
}
/// Implements Scheduler such that the worker with the shortest queue will be given the task
struct NaiveQueue;
impl Scheduler for NaiveQueue {
    fn schedule(&mut self, task: Task, laborers:Vec<WorkerHandle>){
        loop {
            let (send, recv) = mpsc::channel();
            let mut i:usize = 0;
            for laborer in &laborers {
                laborer.channel.send(Task::GiveInfo(send.clone(),InfoRequest::QueueSize(i)));
            }
            let mut replies = Vec::with_capacity(laborers.len());
            for _ in 0..laborers.len() {
                replies.push(recv.try_recv());
            }
            let choice:Option<InfoResponse> = replies.into_iter()
                .map(Result::ok)
                .filter_map(|v|v)
                .min_by_key(|response|{
                    // in the future it won't be irrefutable
                    #[allow(irrefutable_let_patterns)]
                    if let InfoResponse::QueueSize(_, size) = *response{
                        size
                    }
                    else {
                        // if we get back something we don't expect it'll get handled later so
                        // let's just make sure it isn't picked here
                        usize::MAX
                    }
                });
            if let Some(InfoResponse::QueueSize(Id,_)) = choice{
                laborers[Id].channel.send(task);
                break;
            }
        }
    }
}
