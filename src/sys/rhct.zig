const acpi = @import("acpi.zig");

pub const Rhct = extern struct {
    header: acpi.SDTHeader,
    flags: u32,
    timer_hz: u64,
    node_count: u32,
    nodes_offset: u32,
    node: RhctNodeBase,
};

pub const RhctNodeBase = extern struct {
    node_type: NodeType,
    length: u16,
    rev: u16,
};

pub const NodeType = enum(u16) {
    ISAString = 0,
    CMO = 1,
    MMU = 2,
    HartInfo = 65535,
};
