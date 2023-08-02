const acpi = @import("acpi.zig");

pub const Rhct = extern struct {
    header: acpi.SDTHeader,
    flags: RhctFlags,
    timer_hz: u64,
    node_count: u32,
    nodes_offset: u32,
    node: RhctNodeBase,

    pub fn first_node(self: *align(4) const Rhct) *const RhctNodeBase {
        var node_addr = @ptrToInt(self) + self.nodes_offset;
        var node_ptr = @intToPtr(*const RhctNodeBase, node_addr);

        return node_ptr;
    }

    pub fn find_node(self: *align(4) const Rhct, node_type: NodeType) ?*const RhctNodeBase {
        var node = self.first_node();

        var idx: usize = 0;
        while (idx < self.node_count) {
            if (node.node_type == node_type) {
                return node;
            } else {
                node = node.next_block();
            }
            idx += 1;
        }

        return null;
    }
};

pub const RhctFlags = packed struct {
    timer_cant_wake: bool,
    _res: u31,
};

pub const RhctNodeBase = extern struct {
    node_type: NodeType,
    length: u16,
    rev: u16,

    pub fn next_block(self: *const RhctNodeBase) *const RhctNodeBase {
        var base = @ptrToInt(self);
        var ptr = base + self.length;

        return @intToPtr(*const RhctNodeBase, ptr);
    }
};

pub const NodeType = enum(u16) {
    ISAString = 0,
    CMO = 1,
    MMU = 2,
    HartInfo = 65535,
};

pub const ISAStrNode = extern struct {
    base: RhctNodeBase,
    isa_len: u16,
    isa_str: u8,

    pub fn get_str(self: *const ISAStrNode) []const u8 {
        var base = @ptrToInt(&self.isa_str);
        var ptr = @intToPtr([*]const u8, base);
        var slice = ptr[0..self.isa_len];

        return slice;
    }
};

pub const MMUNode = extern struct {
    base: RhctNodeBase,
    _res: u8,
    mmu_type: MMUType,
};

pub const MMUType = enum(u8) {
    Sv39 = 0,
    Sv48 = 1,
    Sv57 = 2,
};
