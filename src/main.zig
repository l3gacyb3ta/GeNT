// © 2023 Lilly & GeNT Developers archaic.archea@gmail.com
// License: [CNPL](https://git.pixie.town/thufie/npl-builder/raw/branch/main/cnpl.md)

const std = @import("std");
const limine = @import("libs/limine.zig");
const fb_lib = @import("io/framebuffer/framebuffer.zig");
const pmm = @import("mem/pmm.zig");
const gentlib = @import("lib.zig");

export var memory_map = limine.MemoryMapRequest{};
export var fb = limine.FramebufferRequest{};
export var hhdm_req = limine.HhdmRequest{};
export var bootinfo = limine.BootloaderInfoRequest{};

export fn _init() linksection(".init.initext") callconv(.C) void {
    const hhdm_offset = hhdm_req.response.?.offset;
    var framebuffer = fb_lib.FrameBuffer.from_limine(fb.response.?.framebuffers_ptr[0]);
    var kstdout = gentlib.getStdWriter(framebuffer);
    var stdout = kstdout.writer();

    stdout.print("Booting on {s} v{s}\n\r", .{ bootinfo.response.?.name, bootinfo.response.?.version }) catch {};

    @breakpoint();

    const memory_map_resp = memory_map.response.?.entries();
    var isinit = false;

    for (memory_map_resp) |entry| {
        stdout.print("Adding entry...\n\r", .{}) catch {};

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

    for ("Set up memory map") |character| {
        framebuffer.write_char(character);
        framebuffer.inc_char();
    }
    framebuffer.inc_char_line();
    framebuffer.ret();

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
