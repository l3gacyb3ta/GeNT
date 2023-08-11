use core::mem::MaybeUninit;

pub struct IOTransit {
    location: usize
}

impl IOTransit {
    pub fn new(location: usize) -> Self {
        Self { location }
    }

    pub fn write<T: Sized>(&self, offset: usize, val: T) {
        let ptr = self.location + offset;
        let ptr = ptr as *mut T;

        unsafe {
            ptr.write_volatile(val);
        }
    }

    pub fn read<T: Copy>(&self, offset: usize) -> T {
        let ptr = self.location + offset;
        let ptr = ptr as *mut T;

        unsafe {
            ptr.read_volatile()
        }
    }

    pub fn loc(&self) -> usize {
        self.location
    }

    unsafe fn read_bytes_raw<S: Copy>(&self, dst: *mut S, size: usize, offset: usize) {
        let mut written = 0;
        while written < size {
            dst.add(written).write(self.read::<S>(offset));
            written += 1;
        }
    }

    pub unsafe fn read_serial<T, S: Copy>(&self, offset: usize) -> T {
        let mut uninit = MaybeUninit::<T>::uninit();
        self.read_bytes_raw(uninit.as_mut_ptr().cast::<S>(), core::mem::size_of::<T>(), offset);
        uninit.assume_init()
    }
}

pub struct Transit;

impl aml::Handler for Transit {
    fn handle_fatal_error(&self, fatal_type: u8, fatal_code: u32, fatal_arg: u64) {
        panic!("FATAL ERROR: type {} code {} arg {}", fatal_type, fatal_code, fatal_arg);
    }

    fn read_u64(&self, address: usize) -> u64 {
        let ptr = address as *const u64;
        unsafe {
            return *ptr;
        }
    }

    fn read_u32(&self, address: usize) -> u32 {
        let ptr = address as *const u32;
        unsafe {
            return *ptr;
        }
    }

    fn read_u16(&self, address: usize) -> u16 {
        let ptr = address as *const u16;
        unsafe {
            return *ptr;
        }
    }

    fn read_u8(&self, address: usize) -> u8 {
        let ptr = address as *const u8;
        unsafe {
            return *ptr;
        }
    }

    fn write_u64(&mut self, address: usize, value: u64) {
        let ptr = address as *mut u64;
        unsafe {
            *ptr = value;
        }
    }

    fn write_u32(&mut self, address: usize, value: u32) {
        let ptr = address as *mut u32;
        unsafe {
            *ptr = value;
        }
    }

    fn write_u16(&mut self, address: usize, value: u16) {
        let ptr = address as *mut u16;
        unsafe {
            *ptr = value;
        }
    }

    fn write_u8(&mut self, address: usize, value: u8) {
        let ptr = address as *mut u8;
        unsafe {
            *ptr = value;
        }
    }

    fn read_pci_u32(&self, _segment: u16, _bus: u8, _device: u8, _function: u8, _offset: u16) -> u32 {
        panic!("PCI not implemented");
    }

    fn read_pci_u16(&self, _segment: u16, _bus: u8, _device: u8, _function: u8, _offset: u16) -> u16 {
        panic!("PCI not implemented");
    }

    fn read_pci_u8(&self, _segment: u16, _bus: u8, _device: u8, _function: u8, _offset: u16) -> u8 {
        panic!("PCI not implemented");
    }

    fn write_pci_u32(&self, _segment: u16, _bus: u8, _device: u8, _function: u8, _offset: u16, _value: u32) {
        panic!("PCI not implemented");
    }

    fn write_pci_u16(&self, _segment: u16, _bus: u8, _device: u8, _function: u8, _offset: u16, _value: u16) {
        panic!("PCI not implemented");
    }

    fn write_pci_u8(&self, _segment: u16, _bus: u8, _device: u8, _function: u8, _offset: u16, _value: u8) {
        panic!("PCI not implemented");
    }

    fn read_io_u32(&self, _port: u16) -> u32 {
        panic!("No IO ports");
    }

    fn read_io_u16(&self, _port: u16) -> u16 {
        panic!("No IO ports");
    }

    fn read_io_u8(&self, _port: u16) -> u8 {
        panic!("No IO ports");
    }
    
    fn write_io_u32(&self, _port: u16, _value: u32) {
        panic!("No IO ports");
    }
    
    fn write_io_u16(&self, _port: u16, _value: u16) {
        panic!("No IO ports");
    }
    
    fn write_io_u8(&self, _port: u16, _value: u8) {
        panic!("No IO ports");
    }
}