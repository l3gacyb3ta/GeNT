const acpi = @import("acpi.zig");

pub const Bgrt = extern struct {
    header: acpi.SDTHeader,
    ver: u16,
    status: packed struct {
        displayed: bool,
        orientation: Orientation,
        res: u5,
    },
    img_type: ImgType,
    img_addr: u64,
    img_off_x: u32,
    img_off_y: u32,
};

pub const ImgType = enum(u8) {
    Bitmap = 0,
};

pub const Orientation = enum(u2) {
    None = 0,
    Ninety = 1,
    OneEighty = 2,
    TwoSeventy = 3,
};
