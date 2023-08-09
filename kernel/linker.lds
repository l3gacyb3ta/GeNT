/*
    // © 2023 Lilly & GeNT Developers archaic.archea@gmail.com
    // License: [CNPL](https://git.pixie.town/thufie/npl-builder/raw/branch/main/cnpl.md)
*/

ENTRY(_boot)

PHDRS
{
    text    PT_LOAD    FLAGS((1 << 0) | (1 << 2)) ; /* Execute + Read */
    rodata  PT_LOAD    FLAGS((1 << 2)) ;            /* Read only */
    data    PT_LOAD    FLAGS((1 << 1) | (1 << 2)) ; /* Write + Read */
    
    ptext    PT_LOAD    FLAGS((1 << 0) | (1 << 2)) ; /* Execute + Read */
    prodata  PT_LOAD    FLAGS((1 << 2)) ;            /* Read only */
    pdata    PT_LOAD    FLAGS((1 << 1) | (1 << 2)) ; /* Write + Read */
    
    initext    PT_LOAD    FLAGS((1 << 0) | (1 << 2)) ; /* Execute + Read */
    inirodata  PT_LOAD    FLAGS((1 << 2)) ;            /* Read only */
    inidata    PT_LOAD    FLAGS((1 << 1) | (1 << 2)) ; /* Write + Read */
    tdata    PT_LOAD    FLAGS((1 << 2)) ;            /* Read only */
}

SECTIONS
{
    . = 0xffffffff80000000 + SIZEOF_HEADERS;

    PROVIDE(__executable_start = .);
        PROVIDE(__nonpageable_start = .);
            .rodata                 : ALIGN(4096) { *(.rodata .rodata.*) } :rodata
            .text                   : ALIGN(4096) { *(.text .text.*) } :text
            PROVIDE(__global_pointer = .);
            .data                   : ALIGN(4096) { *(.data .data.*) *(.sdata .sdata.*) } :data
            .bss                    : ALIGN(4096) { 
                *(.sbss .sbss.*) 
                *(.dynbss) 
                *(.bss .bss.*) 
                . += 0x4000;
                PROVIDE(__stack_top = .);
            } :data
        PROVIDE(__nonpageable_end = .);

        PROVIDE(__pageable_start = .);
            .pdata                  : ALIGN(4096) { *(.pdata .pdata.*) } :pdata
        PROVIDE(__pageable_end = .);

        /*
            Init stuff will ALWAYS be dropped as soon as initialization is exited
        */
        PROVIDE(__init_start = .);
            .initext                : ALIGN(4096) { *(.initext .initext.*) } :initext
            PROVIDE(__tdata_start = .);
                .tdata              : ALIGN(4096) { *(.tdata .tdata.*) } :tdata
                .tbss               : ALIGN(4096) { *(.tbss .tbss.*) } :tdata
            PROVIDE(__tdata_end = .);
        PROVIDE(__init_end = .);
    PROVIDE(__executable_end = .);
}
