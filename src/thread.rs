use crate::state_management::*;

enum ThreadStatus {
    Running,
    Frozen(Box<dyn ThreadState>),
    Awaiting(Box<dyn ThreadState>, Box<dyn Fn()->bool>)
}
pub struct SpinThread{
    status:ThreadStatus
}

unsafe impl Send for ThreadStatus{}
unsafe impl Send for SpinThread{}
unsafe impl Sync for ThreadStatus{}
unsafe impl Sync for SpinThread{}
