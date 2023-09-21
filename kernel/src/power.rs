static SYSCON: AtomicPtr<Syscon> = AtomicPtr::new(0x100_000 as *mut Syscon);

use core::sync::atomic::{AtomicPtr, Ordering};


#[repr(C)]
struct Syscon {
    data_register: u16
}
impl Syscon {
    pub fn poweroff(&mut self) {
        self.data_register = 0x5555;
    }
    pub fn reboot(&mut self) {
        self.data_register = 0x7777;
    }
}

pub fn reboot() {
    unsafe {
        (*SYSCON.load(Ordering::Relaxed)).reboot();
    }
}

pub fn poweroff() {
    unsafe {
        (*SYSCON.load(Ordering::Relaxed)).poweroff();
    }
}