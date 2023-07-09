pub const PageTable = [512]PageTableEntry;

pub const PageTableEntry = packed struct {
    valid: bool,
    read: bool,
    write: bool,
    execute: bool,
    user: bool,
    global: bool,
    accessed: bool,
    dirty: bool,
    paged: bool,
    _unused: bool,
    ppn_swapid: packed union {
        swapid: u44,
        ppn: packed struct {
            ppn0: u9,
            ppn1: u9,
            ppn2: u9,
            ppn3: u9,
            ppn4: u8,

            pub fn index(self: @Type(PageTableEntry.ppn_swapid.ppn), idx: usize) u9 {
                switch (idx) {
                    0 => return self.ppn0,
                    1 => return self.ppn1,
                    2 => return self.ppn2,
                    3 => return self.ppn3,
                    4 => return @as(u9, self.ppn4),
                    _ => return 0,
                }
            }
        },
    },
    _reserved: u7,
    pbmt: MemoryType,
    n: bool,
};

const MemoryType = enum(u2) {
    pma = 0, // Normal memory type
    nc = 1, // Non cached
    io = 2, // IO memory
    unused = 3, // reserved for standard use in the riscv spec
};
