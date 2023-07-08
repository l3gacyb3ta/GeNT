// © 2023 Lilly & GeNT Developers archaic.archea@gmail.com
// License: [CNPL](https://git.pixie.town/thufie/npl-builder/raw/branch/main/cnpl.md)

const limine = @import("../../libs/limine.zig");
const font = @import("font.zig");

pub const FrameBuffer = struct {
    bytes: [*]u8,
    width: usize,
    height: usize,
    pitch: usize,
    cur_x: usize = 0,
    cur_y: usize = 0,

    pub fn from_limine(fb: *const limine.Framebuffer) FrameBuffer {
        return FrameBuffer{
            .bytes = fb.address,
            .width = fb.width,
            .height = fb.height,
            .pitch = fb.pitch,
        };
    }

    pub fn write_pix(self: *const FrameBuffer, rgba: [4]u8) void {
        const offset = (self.cur_x * 4) + (self.cur_y * self.pitch);

        self.bytes[offset] = rgba[0];
        self.bytes[offset + 1] = rgba[1];
        self.bytes[offset + 2] = rgba[2];
        self.bytes[offset + 3] = rgba[3];
    }

    pub fn write_char(self: *const FrameBuffer, character: u8) void {
        const index = @intCast(usize, character);
        const char_bitmap = font.BASIC[index];

        var byte_index: usize = 0;
        while (byte_index < 8) {
            const byte = char_bitmap[byte_index];

            var bit_index: usize = 0;
            while (bit_index < 8) {
                const bit = (byte >> @intCast(u3, bit_index)) & 0b1;
                const byte_offset = self.coord_to_byteoffset(self.cur_x + bit_index, self.cur_y + byte_index);
                var val: u8 = 0;

                if (bit == 1) {
                    val = 0xff;
                }

                self.bytes[byte_offset] = val;
                self.bytes[byte_offset + 1] = val;
                self.bytes[byte_offset + 2] = val;
                self.bytes[byte_offset + 3] = val;

                bit_index += 1;
            }

            byte_index += 1;
        }
    }

    pub fn inc_pix(self: *FrameBuffer) void {
        self.cur_x += 1;
        if (self.cur_x == self.width) {
            self.cur_x = 0;
            self.cur_y += 1;
            if (self.cur_y == self.height) {
                self.cur_y = 0;
            }
        }
    }

    pub fn inc_char(self: *FrameBuffer) void {
        self.cur_x += 8;
        if (self.cur_x >= self.width) {
            self.cur_x = 0;
            self.cur_y += 8;
            if (self.cur_y >= self.height) {
                self.cur_y = 0;
            }
        }
    }

    pub fn inc_char_line(self: *FrameBuffer) void {
        self.cur_y += 8;
        if (self.cur_y >= self.width) {
            self.cur_y = 0;
        }
    }

    pub fn ret(self: *FrameBuffer) void {
        self.cur_x = 0;
    }

    pub fn pixels(self: *const FrameBuffer) usize {
        return self.height * self.width;
    }

    fn coord_to_byteoffset(self: *const FrameBuffer, x: usize, y: usize) usize {
        const offset = (x * 4) + (y * self.pitch);

        return offset;
    }
};
