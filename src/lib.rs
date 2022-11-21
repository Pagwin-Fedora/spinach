
/// ProgState is a trait that a struct describing the current state of the program should implement
/// in order to be used to restore the state upon resuming a task including jumping back to the
/// appropriate point in the program
trait ProgState{
    unsafe fn restore(self);
}
struct SpinRuntime{
    
}
struct SpinThread{
    
}
