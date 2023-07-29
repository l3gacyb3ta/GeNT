// © 2023 Lilly & GeNT Developers archaic.archea@gmail.com
// License: [CNPL](https://git.pixie.town/thufie/npl-builder/raw/branch/main/cnpl.md)

const std = @import("std");
const limine = @import("libs/limine.zig");
const fb_lib = @import("io/framebuffer/framebuffer.zig");
const pmm = @import("mem/pmm.zig");
const gentlib = @import("lib.zig");
const acpi = @import("sys/acpi.zig");

export var memory_map = limine.MemoryMapRequest{};
export var fb = limine.FramebufferRequest{};
export var hhdm_req = limine.HhdmRequest{};
export var bootinfo = limine.BootloaderInfoRequest{};
export var rsdp_req = limine.RsdpRequest{};

export fn _init() linksection(".init.initext") callconv(.C) void {
    const hhdm_offset = hhdm_req.response.?.offset;
    var framebuffer = fb_lib.FrameBuffer.from_limine(fb.response.?.framebuffers_ptr[0]);
    var kstdout = gentlib.getStdWriter(framebuffer);
    var stdout = kstdout.writer();

    stdout.print("Booting on {s} v{s}\n\r", .{ bootinfo.response.?.name, bootinfo.response.?.version }) catch {};

    const memory_map_resp = memory_map.response.?.entries();
    var isinit = false;

    for (memory_map_resp) |entry| {
        var lock = pmm.FREE_MEM.lockOrPanic();
        defer lock.unlock();

        if (entry.kind == limine.MemoryMapEntryType.usable) {
            const base = entry.base + hhdm_offset;
            const length = entry.length;

            var offset: usize = 0;
            while (offset < length) {
                const addr = @intToPtr(*void, base + offset);

                if (!isinit) {
                    lock.deref().init(addr);
                } else {
                    lock.deref().push(addr);
                }

                offset += 4096;
            }
        }
    }

    stdout.print("Set up memory map\n\r", .{}) catch {};

    // Test ACPI stuff
    const rsdp: *const acpi.Rsdp = @ptrCast(*const acpi.Rsdp, rsdp_req.response.?.address);

    if (rsdp.is_valid()) {
        stdout.print("RSDP valid\n\r", .{}) catch {};
    } else {
        stdout.print("RSDP is not 64 bits, aborting\n\r", .{}) catch {};
        while (true) {}
    }

    stdout.print("OEMID: {s}\n\r", .{rsdp.oemid}) catch {};

    const xsdt = rsdp.xsdt_addr;
    if (xsdt.is_valid()) {
        stdout.print("XSDT valid\n\r", .{}) catch {};
    } else {
        stdout.print("XSDT sig invalid\n\r", .{}) catch {};
        while (true) {}
    }

    const entries = xsdt.entry_count();

    stdout.print("Found {} SDT entries\n\r", .{entries}) catch {};

    var sdt_idx: usize = 0;
    while (sdt_idx < entries) {
        var sdts = xsdt.sdts();
        stdout.print("Entry {*}\n\r", .{sdts[sdt_idx]}) catch {};
        sdt_idx += 1;
    }

    //TODO: Set up VMM

    main();
}

fn main() void {
    //TODO: free init sections

    while (true) {}
}

fn panic(msg: []const u8, trace_opt: ?*std.builtin.StackTrace, ret_addr_opt: ?usize) noreturn {
    _ = ret_addr_opt;
    _ = trace_opt;
    _ = msg;

    var framebuffer = fb_lib.FrameBuffer.from_limine(fb.response.?.framebuffers_ptr[0]);

    var pixel_index = 0;
    while (pixel_index < framebuffer.pixels()) {
        framebuffer.write_pix(.{ 0xff, 0x00, 0x00, 0xff });
    }
}
