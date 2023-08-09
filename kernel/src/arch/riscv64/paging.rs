use crate::println;

pub fn get_root_table() -> RootTable {
    let satp_raw: usize;
    unsafe {
        core::arch::asm!(
            "csrr {satp}, satp",
            satp = out(reg) satp_raw,
        );
    }

    let satp: super::csr::Satp = unsafe {core::mem::transmute(satp_raw)};

    return RootTable(satp.phys().to_virt().to_mut_ptr(), satp.mode());
}

pub enum Mode {
    Bare = 0,
    Sv39 = 8,
    Sv48 = 9,
    Sv57 = 10,
    Sv64 = 11,
}

impl Mode {
    pub fn to_level(&self) -> usize {
        use Mode::*;

        match self {
            Bare => 0,
            Sv39 => 3,
            Sv48 => 4,
            Sv57 => 5,
            Sv64 => panic!("Sv64 is not valid")
        }
    }

    pub fn max_size(&self) -> PageSize {
        use Mode::*;

        match self {
            Bare => PageSize::None,
            Sv39 => PageSize::Kilopage,
            Sv48 => PageSize::Terapage,
            Sv57 => PageSize::Petapage,
            Sv64 => panic!("Sv64 is undefined as of implementation"),
        }
    }

    pub fn higher_half(&self) -> usize {
        use Mode::*;

        match self {
            Bare => 0,
            Sv39 => 0xffffffc000000000,
            Sv48 => 0xffff800000000000,
            Sv57 => 0xff00000000000000,
            Sv64 => panic!("Sv64 is undefined as of implementation")
        }
    }
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
pub enum PageSize {
    Petapage = 0x1000_0000_0000,
    Terapage = 0x80_0000_0000,
    Gigapage = 0x4000_0000,
    Megapage = 0x20_0000,
    Kilopage = 0x1000,
    None =     0x0,
}

impl PageSize {
    pub fn from_level(level: usize) -> Self {
        match level {
            0 => Self::Kilopage,
            1 => Self::Megapage,
            2 => Self::Gigapage,
            3 => Self::Terapage,
            4 => Self::Petapage,
            _ => panic!("Invalid page level: {}", level)
        }
    }

    pub fn to_level(&self) -> usize {
        use PageSize::*;

        match self {
            Kilopage => 0,
            Megapage => 1,
            Gigapage => 2,
            Terapage => 3,
            Petapage => 4,
            None => 0,
        }
    }

    pub fn from_size(size: usize) -> Self {
        match size {
            0 => Self::None,
            0x1000 => Self::Kilopage,
            0x200000 => Self::Megapage,
            0x40000000 => Self::Gigapage,
            0x8000000000 => Self::Terapage,
            0x100000000000 => Self::Petapage,
            _ => panic!("Unkown page size {}", size)
        }
    }
}

pub struct PagePermissions {
    pub read: bool,
    pub write: bool,
    pub execute: bool,
    pub user: bool,
    pub global: bool,
}

#[derive(Debug)]
pub enum PageError {
    MappingExists(PageTableEntry),
    InvalidSize,
}

pub struct RootTable(*mut PageTable, Mode);

impl RootTable {
    pub unsafe fn map(
        &mut self, 
        vaddr: crate::mem::VirtualAddress, 
        paddr: crate::mem::PhysicalAddress, 
        perms: PagePermissions, 
        size: PageSize
    ) -> Result<(), PageError> {
        let mut cur_level = self.1.to_level();
        let mut table = self.0;

        if size > self.1.max_size() {
            return Err(PageError::InvalidSize);
        }

        while cur_level != size.to_level() {
            let entry = &mut (*table)[vaddr.vpn(cur_level) as usize];

            match entry.entry() {
                Entry::Table(next_table) => {
                    table = next_table.cast_mut();
                    cur_level -= 1;
                },
                Entry::Page(_page) => {
                    return Err(PageError::MappingExists(*entry));
                },
                Entry::Invalid => {
                    let table_paddr = crate::mem::PHYS.alloc(0x1000, vmem::AllocStrategy::NextFit).unwrap();
                    let table_paddr = crate::mem::PhysicalAddress::new(table_paddr);

                    entry.0 = 0;

                    entry.set_phys(table_paddr);
                    entry.set_valid(true);

                    table = table_paddr.to_virt().to_mut_ptr();
                }
            }
        }

        let entry = &mut (*table)[vaddr.vpn(cur_level) as usize];

        if entry.valid() {
            return Err(PageError::MappingExists(*entry));
        } else {
            entry.0 = 0;
            entry.set_phys(paddr);
            entry.set_read(perms.read);
            entry.set_write(perms.write);
            entry.set_exec(perms.execute);
            entry.set_user(perms.user);
            entry.set_global(perms.global);

            entry.set_valid(true);

            Ok(())
        }
    }

    /// Finds the lowest entry in that virtual address, returns page level, and entry
    pub fn read(&self, vaddr: crate::mem::VirtualAddress) -> (Entry, usize) {
        let mut cur_level = self.1.to_level();
        let mut table = self.0.cast_const();

        loop {
            unsafe {
                let entry = (*table)[vaddr.vpn(cur_level) as usize];
                
                match entry.entry() {
                    Entry::Table(next_table) => {
                        println!("Found table");
                        table = next_table;
                    },
                    _ => {
                        println!("Level: {}", cur_level);
                        println!("found entry: 0x{:x} at index: {}", entry.0, vaddr.vpn(cur_level));
                        return (entry.entry(), cur_level)
                    }
                }

                if cur_level == 0 {
                    panic!("Uhhh :clueless:");
                }

                cur_level -= 1;
            }
        }

    }
}

impl RootTable {}

#[repr(transparent)]
pub struct PageTable([PageTableEntry; 512]);

impl core::ops::Index<usize> for PageTable {
    type Output = PageTableEntry;

    fn index(&self, index: usize) -> &Self::Output {
        &self.0[index]
    }
}

impl core::ops::IndexMut<usize> for PageTable {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.0[index]
    }
}

bitfield::bitfield! {
    #[repr(transparent)]
    #[derive(Clone, Copy)]
    pub struct PageTableEntry(u64);
    impl Debug;

    u64;
    pub valid, set_valid: 0;
    read, set_read: 1;
    write, set_write: 2;
    exec, set_exec: 3;
    user, set_user: 4;
    global, set_global: 5;
    accessed, set_accessed: 6;
    dirty, set_dirty: 7;
    rsw, set_rsw: 9, 8;
    ppn, set_ppn: 53, 10;
    reserved, set_reserved: 60, 54;
    pbmt, set_pbmt: 62, 61;
    n, set_n: 63;
}

pub enum Entry {
    Page(*const u8),
    Table(*const PageTable),
    Invalid,
}

impl PageTableEntry {
    pub fn set_phys(&mut self, val: crate::mem::PhysicalAddress) {
        self.set_ppn((val.addr() >> 12) as u64);
    }

    pub fn phys(&self) -> crate::mem::PhysicalAddress {
        crate::mem::PhysicalAddress::new((self.ppn() << 12) as usize)
    }

    pub fn entry(&self) -> Entry {
        let phys = self.phys();

        if self.valid() {
            let addr = phys.addr() + crate::mem::HHDM_OFFSET.load(core::sync::atomic::Ordering::Relaxed);

            if self.read() | self.write() | self.exec() {
                return Entry::Page(addr as *const u8);
            } else {
                return Entry::Table(addr as *const PageTable);
            }
        } else {
            return Entry::Invalid;
        }
    }
}