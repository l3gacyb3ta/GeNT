#!/usr/bin/python3

import os

id = os.fork()
if id == 0:
    os.execlp("curl", "curl", "https://retrage.github.io/edk2-nightly/bin/RELEASERISCV64_VIRT.fd", "-o EDK2.fd")
else:
    os.waitpid(id, 0);

id = os.fork()
if id == 0:
    os.execlp("curl", "curl", "https://retrage.github.io/edk2-nightly/bin/RELEASERISCV64_Shell.efi", "-o EDK2Shell.fd")
else:
    os.waitpid(id, 0);

id = os.fork()
if id == 0:
    #os.execlp("qemu-system-riscv64", "qemu-system-riscv64", "-help")
    print("No qemu args set")
else:
    os.waitpid(id, 0)