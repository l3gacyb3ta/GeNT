use libsa::endian::{BigEndianU64, BigEndianU32};

#[derive(Debug)]
#[repr(packed)]
pub struct RAMFBConfig {
    address: BigEndianU64,
    /// Format, should always be 0x34325241
    _fourcc: BigEndianU32,
    /// Not used?
    _flags: BigEndianU32,
    _width: BigEndianU32,
    height: BigEndianU32,
    stride: BigEndianU32,
}

impl RAMFBConfig {
    pub fn new(
        width: u32,
        height: u32,
    ) -> Self {
        let stride = width * 4;
        let size = crate::arch::paging::PageSize::from_size_ceil(height as usize * stride as usize) as usize;
        let layout = vmem::Layout::new(size);
        let layout = layout.align(size);

        let physaddr = crate::mem::PHYS.alloc_constrained(
            layout, 
            vmem::AllocStrategy::NextFit
        ).unwrap();

        Self { 
            address: BigEndianU64::new(physaddr as _), 
            _fourcc: BigEndianU32::new(0x34325241), 
            _flags: BigEndianU32::new(0x0), 
            _width: BigEndianU32::new(width), 
            height: BigEndianU32::new(height), 
            stride: BigEndianU32::new(stride)
        }
    }

    pub fn addr(&self) -> u64 {
        self.address.get()
    }

    pub fn byte_size(&self) -> usize {
        self.height.get() as usize * self.stride.get() as usize
    }
}
