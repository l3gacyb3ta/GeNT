# © 2023 Lilly & GeNT Developers archaic.archea@gmail.com
# License: [CNPL](https://git.pixie.town/thufie/npl-builder/raw/branch/main/cnpl.md)

.section .initext,"ax",@progbits
.option norvc
 
.type _asm_init, @function
.global _asm_init
_asm_init:
	.cfi_startproc

    .option push
    .option norelax
    lla gp, _global_pointer
    .option pop

	lla t0, trap
	csrw stvec, t0
 
	tail _init

	j hlt_loop
 
	.cfi_endproc

hlt_loop:
	wfi
	j hlt_loop

trap:
	wfi
	j trap