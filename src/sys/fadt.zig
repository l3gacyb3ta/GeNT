const acpi = @import("acpi.zig");

const Fadt = extern struct {
    header: acpi.SDTHeader,
    firmware_ctrl: u32,
    dsdt: u32,
    // field used in ACPI 1.0; no longer in use, for compatibility only
    _res: u8,

    pref_power_management_prof: u8,
    sci_int: u16,
    smi_cmd_port: u32,
    acpi_enable: u8,
    acpi_disable: u8,
    s4bios_req: u8,
    pstate_ctrl: u8,
    pm1a_event_block: u32,
    pm1b_event_block: u32,
    pm1a_control_block: u32,
    pm1b_control_block: u32,
    pm2_control_block: u32,
    pm_timer_block: u32,
    gpe0_block: u32,
    gpe1_block: u32,
    pm1_event_len: u8,
    pm1_ctrl_len: u8,
    pm2_ctrl_len: u8,
    pm_timer_len: u8,
    gpe0_len: u8,
    gpe1_len: u8,
    gpe1_base: u8,
    c_state_ctrl: u8,
    worst_c2_latency: u16,
    worst_c3_latency: u16,
    flush_size: u16,
    flush_stride: u16,
    duty_offset: u8,
    duty_width: u8,
    day_alarm: u8,
    month_alarm: u8,
    cent: u8,

    // reserved in ACPI 1.0; used since ACPI 2.0+
    boot_arch_flags: u16,

    _res2: u8,
    flags: u32,

    reset_reg: GAddressStruct,

    reset_val: u8,
    _res3: [3]u8,

    // 64bit pointers - Available on ACPI 2.0+
    x_firmware_ctrl: u64,
    x_dsdt: u64,

    x_pm1a_event_block: GAddressStruct,
    x_pm1b_event_block: GAddressStruct,
    x_pm1a_ctrl_block: GAddressStruct,
    x_pm1b_ctrl_block: GAddressStruct,
    x_pm2_ctrl_block: GAddressStruct,
    x_pm_timer_block: GAddressStruct,
    x_gpe0_block: GAddressStruct,
    x_gpe1_block: GAddressStruct,
};

const GAddressStruct = extern struct {
    addr_space: AddrSpace,
    bit_width: u8,
    bit_off: u8,
    access_size: AccessSize,
    addr: u64 align(4),
};

const AddrSpace = enum(u8) {
    SysMem = 0,
    SysIO = 1,
    PCIConfig = 2,
    EmbedCtrller = 3,
    SysManBus = 4,
    SysCMOS = 5,
    PCIDevBAR = 6,
    IntPlatMan = 7,
    GenIO = 8,
    GenSerialBus = 9,
    PlatComChan = 10,
};

const AccessSize = enum(u8) {
    Undefined = 0,
    Byte = 1,
    Double = 2,
    Quad = 3,
    Octo = 4,
};

const PowerManagementProfile = enum(u8) {
    Unspecified = 0,
    Desktop = 1,
    Mobile = 2,
    Workstation = 3,
    EnterpriseServer = 4,
    SOHOServer = 5,
    AppliancePC = 6,
    PerformanceServer = 7,
};
