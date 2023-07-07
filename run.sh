if ! test -f "EDK2.fd"; then
    curl https://retrage.github.io/edk2-nightly/bin/RELEASERISCV64_VIRT.fd -o EDK2.fd
fi
truncate EDK2.fd --size 33554432
    
#TODO: Add OS bin
git clone https://github.com/limine-bootloader/limine.git --depth 1 --branch=v5.x-branch-binary 
make -C limine
mkdir -p iso_root
cp -v limine.cfg limine/limine-bios.sys \
      limine/limine-bios-cd.bin limine/limine-uefi-cd.bin iso_root/
mkdir -p iso_root/EFI/BOOT
cp -v limine/BOOT*.EFI iso_root/EFI/BOOT/
xorriso -as mkisofs -b limine-bios-cd.bin \
        -no-emul-boot -boot-load-size 4 -boot-info-table \
        --efi-boot limine-uefi-cd.bin \
        -efi-boot-part --efi-boot-image --protective-msdos-label \
        iso_root -o GeNT.iso
./limine/limine bios-install GeNT.iso

qemu-system-riscv64 \
    -machine virt \
    -cpu rv64,svpbmt=on \
    -smp 1 \
    -m 512M \
    -drive if=pflash,format=raw,unit=1,file=EDK2.fd \
    -global virtio-mmio.force-legacy=false \
    -device ramfb \
    -cdrom GeNT.iso \
    -serial mon:stdio