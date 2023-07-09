const std = @import("std");
const serial = @import("../uart/serial.zig");

pub const KoutErr = error{WriteFailure};

pub const Kstderr = struct {
    address: *volatile serial.Uart16550 = @intToPtr(*volatile serial.Uart16550, 0x1000_0000),

    const Writer = std.io.Writer(
        *Kstderr,
        KoutErr,
        derefWrite,
    );

    fn derefWrite(
        self: *Kstderr,
        string: []const u8,
    ) KoutErr!usize {
        for (string) |char| {
            self.address.write(char);
        }

        return string.len;
    }

    pub fn writer(self: *Kstderr) Writer {
        return .{ .context = self };
    }
};
