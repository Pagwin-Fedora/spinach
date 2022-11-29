use std::arch::asm;
/// ProgState is a trait that a struct describing the current state of the program should implement
/// in order to be used to restore the state upon resuming a task including jumping back to the
/// appropriate point in the program
trait ThreadState{
    #[allow(improper_ctypes_definitions)]
    unsafe extern "C" fn restore(self);
}
struct FuncResume<T,F:FnOnce(T)>{
    state: T,
    call: F
}
impl<T,F:FnOnce(T)> ThreadState for FuncResume<T,F> {
    #[allow(improper_ctypes_definitions)]
     unsafe extern "C" fn restore(self) {
        (self.call)(self.state)
    }
}
/// Implements ThreadState to restore registers to a specified state
/// Warning: rax is clobbered for the sake of changing control flow upon restoration when using new
#[derive(Clone,Copy)]
struct RegisterReset{
    rax: u64,
    rbx: u64,
    rcx: u64,
    rdx: u64,
    rsi: u64,
    rdi: u64,
    rbp: u64,
    rsp: u64,
    r8:  u64,
    r9:  u64,
    r10: u64,
    r11: u64,
    r12: u64,
    r13: u64,
    r14: u64,
    r15: u64,
    rip: u64,
}
impl RegisterReset {
    /// use asm macro to grab all the register values and create a new struct(this code can
    /// definitely be shortened)
    #[allow(improper_ctypes_definitions)]
    extern "C" fn new()-> RegisterReset{
        let rax: u64;
        let rbx: u64;
        let rcx: u64;
        let rdx: u64;
        let rsi: u64;
        let rdi: u64;
        let rbp: u64;
        let rsp: u64;
        let r8:  u64;
        let r9:  u64;
        let r10: u64;
        let r11: u64;
        let r12: u64;
        let r13: u64;
        let r14: u64;
        let r15: u64;
        let rip: u64;
        unsafe {
            asm!("mov {}, rax",
                 "mov {}, rbx",
                 "mov {}, rcx",
                 "mov {}, rdx",
                 "mov {}, rsi",
                 "mov {}, rdi",
                 "mov {}, rbp",
                 "mov {}, rsp",
                 "mov {}, r8",
                 "mov {}, r9",
                 "mov {}, r10",
                 "mov {}, r11",
                 "mov {}, r12",
                 "mov {}, r13",
                 "mov {}, r14",
                 "mov {}, r15",
                 out(reg) rax,
                 out(reg) rbx,
                 out(reg) rcx,
                 out(reg) rdx,
                 out(reg) rsi,
                 out(reg) rdi,
                 out(reg) rbp,
                 out(reg) rsp,
                 out(reg) r8,
                 out(reg) r9,
                 out(reg) r10,
                 out(reg) r11,
                 out(reg) r12,
                 out(reg) r13,
                 out(reg) r14,
                 out(reg) r15);
            // set rip to the return location from new
            rip = *(rsi as *mut u64).offset(1)
        }
        Self {
            rax,
            rbx,
            rcx,
            rdx,
            rsi,
            rdi,
            rbp,
            rsp,
            r8,
            r9,
            r10,
            r11,
            r12,
            r13,
            r14,
            r15,
            rip
        }
    }
}
impl ThreadState for RegisterReset {
    // should technically return ! but the never type is experimental so can't
    #[allow(improper_ctypes_definitions)]
    unsafe extern "C" fn restore(self){
        asm!("mov rax, {}",
             "mov rbx, {}",
             "mov rcx, {}",
             "mov rdx, {}",
             "mov rsi, {}",
             "mov rdi, {}",
             "mov rbp, {}",
             "mov rsp, {}",
             "mov r8, {}",
             "mov r9, {}",
             "mov r10, {}",
             "mov r11, {}",
             "mov r12, {}",
             "mov r13, {}",
             "mov r14, {}",
             "mov r15, {}",
             // NOTE: changing rip must ALWAYS BE DONE AFTER EVERYTHING ELSE because it changes
             // control flow away from here
             "mov rip, {}",
             in(reg) self.rax,
             in(reg) self.rbx,
             in(reg) self.rcx,
             in(reg) self.rdx,
             in(reg) self.rsi,
             in(reg) self.rdi,
             in(reg) self.rbp,
             in(reg) self.rsp,
             in(reg) self.r8,
             in(reg) self.r9,
             in(reg) self.r10,
             in(reg) self.r11,
             in(reg) self.r12,
             in(reg) self.r13,
             in(reg) self.r14,
             in(reg) self.r15,
             in(reg) self.rip);
    }
}

struct MemRegReset{
    //pointer to a null type due to type of value we're copying being unknown
    memory: Vec<(*mut (), Box<[u8]>)>,
    regs: RegisterReset
}
impl MemRegReset {
    #[allow(improper_ctypes_definitions)]
    extern "C" fn new()->MemRegReset{
        let tmp_rsp:*mut u64;
        let tmp_rip: u64;
        unsafe {
            asm!("mov {}, rsp",out(reg) tmp_rsp);
            tmp_rip = *tmp_rsp.offset(1);
        }
        let mut regs = RegisterReset::new();
        regs.rip = tmp_rip;
        MemRegReset { memory: Vec::new(), regs }
    }
}
impl ThreadState for MemRegReset{
    #[allow(improper_ctypes_definitions)]
    unsafe extern "C" fn restore(self) {
        //restore all parts of memory we've got
        for (loc,data) in self.memory{
            data.as_ptr().copy_to(loc as *mut u8, data.len())
        }
        self.regs.restore()
    }
}
struct SpinRuntime{
    
}
struct SpinThread{
    
}
