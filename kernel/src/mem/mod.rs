use core::sync::atomic;

pub static PHYS: vmem::Vmem = vmem::Vmem::new(alloc::borrow::Cow::Borrowed("PHYSMEM"), 4096, None);
pub static VIRT: vmem::Vmem = vmem::Vmem::new(alloc::borrow::Cow::Borrowed("VIRTMEM"), 4096, None);

pub static HHDM_OFFSET: atomic::AtomicUsize = atomic::AtomicUsize::new(0);

#[derive(Clone, Copy)]
pub struct PhysicalAddress(usize);

impl PhysicalAddress {
    pub fn new(val: usize) -> Self {
        Self(val)
    }
    
    pub fn addr(&self) -> usize {
        self.0
    }

    pub fn to_virt(&self) -> VirtualAddress {
        VirtualAddress::new(self.0 + HHDM_OFFSET.load(atomic::Ordering::Relaxed))
    }
}

pub type VirtualAddress = crate::arch::mem::VirtualAddress;