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
                stderr.print("\n", .{}) catch {};
                self.framebufferwriter.inc_char_line();
            } else if (char == '\r') {
                stderr.print("\r", .{}) catch {};
                self.framebufferwriter.ret();
            } else {
                stderr.print("{c}", .{char}) catch {};
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
