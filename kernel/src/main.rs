#![no_main]
#![no_std]
#![feature(
    naked_functions,
    int_roundings
)]

extern crate alloc;

use alloc::boxed::Box;
use gent_kern::println;

static RSDP: limine::RsdpRequest = limine::RsdpRequest::new();
static MMAP: limine::MemoryMapRequest = limine::MemoryMapRequest::new();

#[link_section = ".initext"]
extern "C" fn kinit() -> ! {
    println!("Starting kernel");
    println!("Mode: {:?}", gent_kern::MODE.response().unwrap().mode());
    unsafe {
        vmem::bootstrap()
    }
    let memory_map = MMAP.response().expect("Failed to get map");
    for entry in memory_map.usable_entries() {
        gent_kern::mem::PHYS.add(entry.base, entry.size).expect("Failed to add entry");
    }

    gent_kern::allocator::init();
    gent_kern::find_upperhalf_mem();
    loop {}
    println!("Memory initialized");

    let xsdt = unsafe {
        acpi::AcpiTables::from_rsdp(
            Handler, 
            RSDP.response().unwrap().rsdp_addr as usize
        )
    }.unwrap();
    println!("XSDT found");

    let fadt = unsafe {
        xsdt.get_sdt::<acpi::fadt::Fadt>(acpi::sdt::Signature::FADT).unwrap().unwrap()
    };

    let dsdt = fadt.dsdt_address().unwrap();

    let aml_handler: Box<dyn aml::Handler> = Box::new(gent_kern::arch::transit::Transit);

    let mut context = aml::AmlContext::new(
        aml_handler, 
        aml::DebugVerbosity::All
    );
    println!("Made AML context");

    unsafe {
        let _dsdt = context.parse_table(
            &*core::ptr::slice_from_raw_parts(
                dsdt as *const u8, 
                20
            )
        ).unwrap();
    }

    println!("Kernel end");
    loop {}
}

#[derive(Clone, Copy)]
struct Handler;

impl acpi::AcpiHandler for Handler {
    unsafe fn map_physical_region<T>(&self, physical_address: usize, size: usize) -> acpi::PhysicalMapping<Self, T> {
        use gent_kern::arch::paging;
        let physical_address = physical_address - gent_kern::mem::HHDM_OFFSET.load(core::sync::atomic::Ordering::Relaxed);

        println!("Mapping physical address 0x{:x}", physical_address);

        let mut root = paging::get_root_table();

        let vaddr = gent_kern::mem::VIRT.alloc(size, vmem::AllocStrategy::NextFit).unwrap();

        let mut mapped_len = 0;

        for addr_offset in (0..size).step_by(4096) {
            let vaddr = gent_kern::mem::VirtualAddress::new(vaddr + addr_offset);
            let paddr = gent_kern::mem::PhysicalAddress::new(physical_address + addr_offset);

            let size = size.div_ceil(0x1000) * 0x1000;

            root.map(
                vaddr, 
                paddr, 
                paging::PagePermissions {
                    read: true,
                    write: true,
                    execute: false,
                    user: false,
                    global: false,
                }, 
                paging::PageSize::from_size(size)
            ).unwrap_or_else(|err| {

                match err {
                    paging::PageError::InvalidSize => {
                        panic!("Failed to map due to invalid size");
                    },
                    paging::PageError::MappingExists(entry) => {
                        println!(
                            "Failed to map due to existing mapping: {:#x?}\nAttempted mapping: v0x{:x} to p0x{:x}\nOriginal address: p0x{:x}", 
                            err, 
                            vaddr.addr(), 
                            paddr.addr(),
                            entry.phys().addr()
                        );
                    }
                }
            });

            mapped_len += 0x1000;
        }

        acpi::PhysicalMapping::new(
            physical_address, 
            core::ptr::NonNull::new(vaddr as *mut T).unwrap(), 
            size, 
            mapped_len, 
            *self
        )
    }

    fn unmap_physical_region<T>(_region: &acpi::PhysicalMapping<Self, T>) {
        panic!("NO UNMAPPING ACPI STUFF YET!!!!!!!!")
    }
}

#[naked]
#[no_mangle]
#[link_section = ".initext"]
unsafe extern "C" fn _boot() -> ! {
    core::arch::asm!("
        csrw sie, zero
        csrci sstatus, 2
        
        .option push
        .option norelax
        lla gp, __global_pointer
        .option pop

        lla sp, __stack_top
        lla t0, stvec_trap_shim
        csrw stvec, t0

        2:
            j {}
    ", sym kinit, options(noreturn));
}

#[naked]
#[no_mangle]
unsafe extern "C" fn stvec_trap_shim() -> ! {
    core::arch::asm!("
        1:
            wfi
            j 1b
    ", options(noreturn));
}

#[panic_handler]
fn panic(info: &core::panic::PanicInfo) -> ! {
    println!("Panic: {:#?}", info);
    loop {}
}