set architecture i386:x86-64
file target/debug/x64/EFI/BOOT/BOOTX64.EFI
target remote localhost:1234
add-symbol-file target/debug/x64/EFI/BOOT/BOOTX64.EFI 0x3c871000
hbreak *0x3c8719e0
continue
