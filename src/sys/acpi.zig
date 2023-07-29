const std = @import("std");

pub const Rsdp = extern struct {
    sig: [8]u8 align(1),
    checksum: u8 align(1),
    oemid: [6]u8 align(1),
    rev: u8 align(1),
    rsdt_addr: u32 align(1),
    len: u32 align(1),
    xsdt_addr: *const Xsdt align(1),
    ext_checksum: u8 align(1),
    res: [3]u8 align(1),

    pub fn is_valid(self: *const Rsdp) bool {
        return self.rev >= 2;
    }
};

pub const Xsdt = extern struct {
    header: SDTHeader,
    sdt_ptr: u64 align(4),

    pub fn entry_count(self: *const Xsdt) usize {
        return (self.header.len - @sizeOf(SDTHeader)) / 8;
    }

    pub fn sdts(self: *const Xsdt) [*]align(4) const *const SDTHeader {
        var ptr_base = @ptrToInt(&self.sdt_ptr);
        var ptr = @intToPtr([*]align(4) const *const SDTHeader, ptr_base);

        return ptr;
    }

    pub fn is_valid(self: *const Xsdt) bool {
        var is_same = true;

        const xsdt_sig = "XSDT";

        var idx: usize = 0;
        while (idx < 4) {
            is_same = is_same and (self.header.sig[idx] == xsdt_sig[idx]);

            idx += 1;
        }

        //return is_same and self.header.is_valid();
        return is_same;
    }
};

pub const SDTHeader = extern struct {
    sig: [4]u8,
    len: u32,
    rev: u8,
    checksum: u8,
    oemid: [6]u8,
    oemtid: [8]u8,
    oemrev: u32,
    creator_id: u32,
    creator_rev: u32,

    pub fn print_header(self: *const SDTHeader, stdout: anytype) !void {
        try stdout.print("sig: {s}\n\r", .{self.sig});
        try stdout.print("len: {}\n\r", .{self.len});
        try stdout.print("rev: {}\n\r", .{self.rev});
        try stdout.print("checksum: {}\n\r", .{self.checksum});
        try stdout.print("oemid: {s}\n\r", .{self.oemid});
        try stdout.print("oemtid: {s}\n\r", .{self.oemtid});
        try stdout.print("oemrev: {}\n\r", .{self.oemrev});
        try stdout.print("creator id: {}\n\r", .{self.creator_id});
        try stdout.print("creator rev: {}\n\r", .{self.creator_rev});
    }
};
