#![no_main]
#![no_std]
#![feature(
    naked_functions,
    int_roundings,
    strict_provenance,
    pointer_byte_offsets
)]

extern crate alloc;

use alloc::boxed::Box;
use gent_kern::println;

static RSDP: limine::RsdpRequest = limine::RsdpRequest::new();
static MMAP: limine::MemoryMapRequest = limine::MemoryMapRequest::new();

#[link_section = ".initext"]
extern "C" fn kinit() -> ! {
    log::set_logger(&gent_kern::uart::Logger).unwrap();
    log::set_max_level(log::LevelFilter::Info);
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
    println!("Memory initialized");
    gent_kern::find_upperhalf_mem();
    println!("Upperhalf found");

    let xsdt = unsafe {
        acpi::AcpiTables::from_rsdp(
            Handler, 
            RSDP.response().unwrap().rsdp_addr as usize - gent_kern::mem::HHDM_OFFSET.load(core::sync::atomic::Ordering::Relaxed)
        )
    }.unwrap();
    println!("XSDT found");

    let aml_handler: Box<dyn aml::Handler> = Box::new(gent_kern::arch::transit::Transit);

    let mut context = aml::AmlContext::new(
        aml_handler, 
        aml::DebugVerbosity::All
    );
    println!("Made AML context");

    unsafe {
        let dsdt = xsdt.dsdt.unwrap();
        context.parse_table(
            &*core::ptr::slice_from_raw_parts(
                dsdt.address as *const u8, 
                dsdt.length as usize
            )
        ).unwrap();

        for ssdt in xsdt.ssdts.iter() {
            context.parse_table(
                &*core::ptr::slice_from_raw_parts(
                    ssdt.address as *const u8, 
                    ssdt.length as usize
                )
            ).unwrap();
        }

        context.initialize_objects().unwrap();
    }

    println!("{:#?}", context.namespace);

    let resources = aml::resource::resource_descriptor_list(
        context.namespace.get_by_path(
            &aml::AmlName::from_str("\\_SB_.FWCF._CRS").unwrap()
        ).unwrap()
    ).unwrap();

    let fw_cfg = &resources[0];

    match fw_cfg {
        aml::resource::Resource::MemoryRange(range) => {
            match range {
                aml::resource::MemoryRangeDescriptor::FixedLocation { 
                    is_writable: _, 
                    base_address, 
                    range_length : _
                } => {
                    let base_address = (*base_address as usize) + gent_kern::mem::HHDM_OFFSET.load(core::sync::atomic::Ordering::Relaxed);

                    // TODO: Figure out how to see if something is Port IO, or Memory IO
                    let fw_cfg = unsafe {gent_kern::dev::fw_cfg::FwCfg::new(gent_kern::arch::global::IOType::Mem(base_address))};

                    let files = fw_cfg.files();

                    for file in files.iter() {
                        println!("File {:#?}", file.name().unwrap());
                    }

                    let ramfb = fw_cfg.lookup("etc/ramfb").unwrap();

                    println!("RAMFB sel: 0x{:x}", ramfb.sel().get());

                    let ramfbcfg = gent_kern::dev::ramfb::RAMFBConfig::new(640, 480);
                    let cfg_arr: [u8; 28] = unsafe {core::mem::transmute(ramfbcfg)};

                    fw_cfg.write_file(ramfb, &cfg_arr).unwrap();

                    let ramfbcfg: gent_kern::dev::ramfb::RAMFBConfig = unsafe {core::mem::transmute(cfg_arr)};

                    for i in 0..ramfbcfg.byte_size() {
                        unsafe {
                            let ptr = ramfbcfg.addr() as *mut u8;

                            ptr.add(i).write_volatile(0xff);
                        }
                    }
                }
            }
        },
        _ => unreachable!()
    }


    println!("Kernel end");
    loop {}
}

#[derive(Clone, Copy)]
struct Handler;

impl acpi::AcpiHandler for Handler {
    unsafe fn map_physical_region<T>(&self, physical_address: usize, size: usize) -> acpi::PhysicalMapping<Self, T> {
        use gent_kern::arch::paging;
        
        // Mask out the bottom bits, they shouldnt be needed in the address
        let physical_address = physical_address & 0xFFFFFFFFFFFFF000;
        // Add the bottom bits from the physical address for edge cases
        // E.g.: size 0x1000, which would require 2 pages normally with address 0x1001, but if we remove the bottom bits, its only 1 page
        let size = size + (physical_address & 0xFFF);

        let mut root = paging::get_root_table();

        let mapped_len = size.div_ceil(0x1000) * 0x1000;

        let vaddr = gent_kern::mem::VIRT.alloc(mapped_len, vmem::AllocStrategy::NextFit).unwrap();

        for addr_offset in (0..size).step_by(4096) {
            let vaddr = gent_kern::mem::VirtualAddress::new(vaddr + addr_offset);
            let paddr = gent_kern::mem::PhysicalAddress::new(physical_address + addr_offset);

            let size = 0x1000;

            root.map(
                vaddr, 
                paddr, 
                paging::PagePermissions {
                    read: true,
                    write: true,
                    execute: false,
                    user: false,
                    global: false,
                    dealloc: false
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
                    },
                    _ => {
                        panic!("{:#?}", err)
                    }
                }
            });
        }

        acpi::PhysicalMapping::new(
            physical_address, 
            core::ptr::NonNull::new(vaddr as *mut T).unwrap(), 
            size, 
            mapped_len, 
            *self
        )
    }

    fn unmap_physical_region<T>(region: &acpi::PhysicalMapping<Self, T>) {
        unsafe {
            use gent_kern::arch::paging;
            let virtual_address = region.virtual_start().addr().get();

            let mut root = paging::get_root_table();

            for addr_offset in (0..region.mapped_length()).step_by(4096) {
                let vaddr = gent_kern::mem::VirtualAddress::new(virtual_address + addr_offset);

                let size = 0x1000;

                root.unmap(
                    vaddr, 
                    paging::PageSize::from_size(size)
                ).unwrap_or_else(|err| {
                    match err {
                        paging::PageError::NoMapping => {
                            panic!("No mapping found");
                        },
                        paging::PageError::UnmappingSizeMismatch => {
                            panic!("Sizes did not match");
                        }
                        _ => {
                            panic!("{:#?}", err)
                        }
                    }
                });
            }

            gent_kern::mem::VIRT.free(region.virtual_start().addr().get(), region.mapped_length());
        }
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