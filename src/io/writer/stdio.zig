const std = @import("std");
const fb = @import("../framebuffer/framebuffer.zig");
const gentlib = @import("../../lib.zig");

pub fn getWriter() void {}

pub const KoutErr = error{WriteFailure};

pub const Kstdout = struct {
    framebufferwriter: fb.FrameBuffer,

    const Writer = std.io.Writer(
        *Kstdout,
        KoutErr,
        derefWrite,
    );

    fn derefWrite(
        self: *Kstdout,
        string: []const u8,
    ) KoutErr!usize {
        var kstderr = gentlib.getStdErr();
        var stderr = kstderr.writer();

        for (string) |char| {
            if (char == '\n') {
                stderr.print("Writing '\\n' to framebuffer\n", .{}) catch {};
                self.framebufferwriter.inc_char_line();
            } else if (char == '\r') {
                stderr.print("Writing '\\r' to framebuffer\n", .{}) catch {};
                self.framebufferwriter.ret();
            } else {
                stderr.print("Writing '{c}'(0x{x}) to framebuffer\n", .{ char, char }) catch {};
                self.framebufferwriter.write_char(char);
                self.framebufferwriter.inc_char();
            }
        }

        return string.len;
    }

    pub fn writer(self: *Kstdout) Writer {
        return .{ .context = self };
    }
};
