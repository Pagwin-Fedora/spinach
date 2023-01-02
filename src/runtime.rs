use crate::thread::*;
use crate::worker::*;

pub struct SpinRuntime{
    threads: Vec<SpinThread>,
    workers: Vec<Worker>
}
