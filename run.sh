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

qemu-system-riscv64 \
    -machine virt \
    -cpu rv64,svpbmt=on \
    -smp 1 \
    -m 512M \
    -drive if=pflash,format=raw,unit=1,file=EDK2.fd \
    -device nvme,serial=deadbeff,drive=disk1 \
    -drive id=disk1,format=raw,if=none,file=fat:rw:./iso_root\
    -global virtio-mmio.force-legacy=false \
    -device ramfb \
    -serial mon:stdio