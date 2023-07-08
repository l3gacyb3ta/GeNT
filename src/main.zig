// © 2023 Lilly & GeNT Developers archaic.archea@gmail.com
// License: [CNPL](https://git.pixie.town/thufie/npl-builder/raw/branch/main/cnpl.md)

const std = @import("std");
const limine = @import("libs/limine.zig");
const fb_lib = @import("framebuffer.zig");

export var fb = limine.FramebufferRequest{};

export fn _init() linksection(".initext") void {
    var framebuffer = fb_lib.FrameBuffer.from_limine(fb.response.?.framebuffers_ptr[0]);

    framebuffer.write_char('a');

    while (true) {}
}
