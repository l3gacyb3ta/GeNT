/*
    // © 2023 Lilly & GeNT Developers archaic.archea@gmail.com
    // License: [CNPL](https://git.pixie.town/thufie/npl-builder/raw/branch/main/cnpl.md)
*/

ENTRY(_init)

KERN_BASE  = 0xfffff80000000000;

SECTIONS
{
    . = KERN_BASE + SIZEOF_HEADERS;

    PROVIDE(__executable_start = .);
        PROVIDE(__nonpageable_start = .);
            .rodata                 : ALIGN(4096) { *(.rodata .rodata.*) }
            .text                   : ALIGN(4096) { *(.text .text.*) }
            .data                   : ALIGN(4096) { *(.data .data.*) *(.sdata .sdata.*) }
            .bss                    : ALIGN(4096) { *(.sbss .sbss.*) *(.dynbss) *(.bss .bss.*) }
        PROVIDE(__nonpageable_end = .);

        PROVIDE(__pageable_start = .);
            .pdata                  : ALIGN(4096) { *(.pdata .pdata.*) }
        PROVIDE(__pageable_end = .);

        /*
            Init stuff will ALWAYS be dropped as soon as initialization is exited
        */
        PROVIDE(__init_start = .);
            PROVIDE(__tdata_start = .);
                .tdata                  : ALIGN(4096) { *(.tdata .tdata.*) }
                .tbss                   : ALIGN(4096) { *(.tbss .tbss.*) }
            PROVIDE(__tdata_end = .);
        PROVIDE(__init_end = .);
    PROVIDE(__executable_end = .);
}