#![no_std]

extern crate alloc;

pub mod allocator;
pub mod arch;
pub mod mem;
pub mod uart;

static HHDM: limine::HhdmRequest = limine::HhdmRequest::new();
pub static MODE: limine::PagingModeRequest = limine::PagingModeRequest::new(
    limine::PagingMode::Sv57, 
    limine::PagingModeRequestFlags::empty()
);

pub fn find_upperhalf_mem() {
    let base = HHDM.response().unwrap().base;

    mem::HHDM_OFFSET.store(
        base,
        core::sync::atomic::Ordering::Relaxed
    );

    let root = arch::paging::get_root_table();


    let mut addr = base;

    //while (addr < 0xffffffff80000000) && (addr >= base) {
        let vaddr = mem::VirtualAddress::new(addr);

        let (entry, level) = root.read(vaddr);

        let size = arch::paging::PageSize::from_level(level) as usize;

        match entry {
            arch::paging::Entry::Invalid => {
                println!("Adding entry at 0x{:x}", vaddr.addr());
                mem::VIRT.add(addr, size).expect("Failed to add virtual entry");
            },
            arch::paging::Entry::Page(_) => {},
            arch::paging::Entry::Table(_) => unreachable!()
        }

        addr += size;
    //}
}