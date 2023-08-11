rm -rf ./.root

cd kernel
cargo build --release
cd ..

if [ $? -eq 0 ]; then
    echo Build success
else
    echo Build failure
    exit
fi

if ! test -f "EDK2.fd"; then
    curl https://retrage.github.io/edk2-nightly/bin/RELEASERISCV64_VIRT.fd -o EDK2.fd
fi
truncate EDK2.fd --size 33554432

if test -f "limine"; then
    echo Grabbing Limine
    git clone https://github.com/limine-bootloader/limine.git --depth 1 --branch=v5.x-branch-binary 
fi 
make -C limine
mkdir -p .root
cp -v kernel/target/riscv64gc-unknown-none-elf/release/gent-kern config/limine.cfg limine/limine-bios.sys \
      limine/limine-bios-cd.bin limine/limine-uefi-cd.bin \
      .root/
mkdir -p .root/EFI/BOOT
cp -v limine/BOOTRISCV64.EFI .root/EFI/BOOT/

qemu-system-riscv64 \
    -machine virt,aclint=on,acpi=on,aia=aplic-imsic \
    -cpu rv64,svpbmt=on \
    -smp 1 \
    -m 4G \
    -drive if=pflash,format=raw,unit=1,file=EDK2.fd \
    -device nvme,serial=deadbeff,drive=disk1 \
    -drive id=disk1,format=raw,if=none,file=fat:rw:./.root \
    -global virtio-mmio.force-legacy=false \
    -device ramfb \
    -serial mon:stdio \
    -d int,trace:fw_cfg_select,trace:fw_cfg_read \
    -D debug.log