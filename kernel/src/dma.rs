use core::marker::PhantomData;

pub struct DmaRange<T: ?Sized> {
    /// Number of `T`s it has
    length: usize,
    virt: usize,
    phys: usize,
    data: PhantomData<T>
}

impl<T: Sized> DmaRange<[T]> {
    pub fn new_many(amount: usize) -> Self {
        let mut root = crate::arch::paging::get_root_table();

        let size = core::mem::size_of::<T>() * amount;

        let phys = crate::mem::PHYS.alloc(size, vmem::AllocStrategy::NextFit).unwrap();
        let virt = crate::mem::VIRT.alloc(size, vmem::AllocStrategy::NextFit).unwrap();

        let map_size = size.div_ceil(0x1000) * 0x1000;
        for offset in (0..map_size).step_by(0x1000) {
            let vaddr = crate::mem::VirtualAddress::new(virt + offset);
            let paddr = crate::mem::PhysicalAddress::new(phys + offset);

            unsafe {
                root.map(
                    vaddr, 
                    paddr, 
                    crate::arch::paging::PagePermissions {
                        read: true,
                        write: true,
                        execute: false,
                        user: false,
                        global: false,
                        dealloc: false,
                    }, 
                    crate::arch::paging::PageSize::Kilopage
                ).unwrap();
            }
        }

        Self { 
            length: amount, 
            virt, 
            phys,
            data: PhantomData 
        }
    }

    pub fn buf(&self) -> &[T] {
        let ptr = self.virt as *mut T;
        let amount = self.length;

        let slice: &[T] = unsafe {core::slice::from_raw_parts(ptr, amount)};

        return slice;
    }

    pub fn buf_mut(&mut self) -> &mut [T] {
        let ptr = self.virt as *mut T;
        let amount = self.length;

        let slice: &mut [T] = unsafe {core::slice::from_raw_parts_mut(ptr, amount)};

        return slice;
    }
}

impl<T> DmaRange<T> {
    pub fn new() -> Self {
        let mut root = crate::arch::paging::get_root_table();

        let size = core::mem::size_of::<T>();

        let phys = crate::mem::PHYS.alloc(size, vmem::AllocStrategy::NextFit).unwrap();
        let virt = crate::mem::VIRT.alloc(size, vmem::AllocStrategy::NextFit).unwrap();

        let map_size = size.div_ceil(0x1000) * 0x1000;
        for offset in (0..map_size).step_by(0x1000) {
            let vaddr = crate::mem::VirtualAddress::new(virt + offset);
            let paddr = crate::mem::PhysicalAddress::new(phys + offset);

            unsafe {
                root.map(
                    vaddr, 
                    paddr, 
                    crate::arch::paging::PagePermissions {
                        read: true,
                        write: true,
                        execute: false,
                        user: false,
                        global: false,
                        dealloc: false,
                    }, 
                    crate::arch::paging::PageSize::Kilopage
                ).unwrap();
            }
        }

        Self { 
            length: 1, 
            virt, 
            phys,
            data: PhantomData 
        }
    }

    /// Takes the self and returns a reference, and physical address
    pub fn leak(self) -> (&'static mut [T], usize) {
        let ptr = self.virt as *mut T;
        let amount = self.length / core::mem::size_of::<T>();

        let slice: &'static mut [T] = unsafe {core::slice::from_raw_parts_mut(ptr, amount)};

        return (slice, self.phys);
    }
}

impl<T: ?Sized> DmaRange<T> {
    pub fn phys(&self) -> usize {
        self.phys
    }
}

impl<T: ?Sized> Drop for DmaRange<T> {
    fn drop(&mut self) {
        unsafe {
            let mut root = crate::arch::paging::get_root_table();

            crate::mem::PHYS.free(self.phys, self.length);
            crate::mem::VIRT.free(self.virt, self.length);

            let size = self.length;
            let virt = self.virt;

            let map_size = size.div_ceil(0x1000) * 0x1000;
            for offset in (0..map_size).step_by(0x1000) {
                let vaddr = crate::mem::VirtualAddress::new(virt + offset);
    
                root.unmap(
                    vaddr, 
                    crate::arch::paging::PageSize::Kilopage
                ).unwrap();
            }
        }
    }
}

impl<T> core::ops::Deref for DmaRange<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        let ptr = self.virt as *const T;

        unsafe {&*ptr}
    }
}

impl<T> core::ops::DerefMut for DmaRange<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        let ptr = self.virt as *mut T;

        unsafe {&mut *ptr}
    }
}