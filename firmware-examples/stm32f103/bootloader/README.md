# STM32F103 Example DFU Bootloader for fwupd

This bootloader is based on the [usbd-dfu](https://github.com/vitalyvb/usbd-dfu)
stack example bootloader published [here](https://github.com/vitalyvb/usbd-dfu-example),
from [vitalyvb](https://github.com/vitalyvb).

It implements the DFUse extensions (DFU 1.1a), and declares a memory map via USB
descriptor string in the following way:

```rust
const MEM_INFO_STRING: &'static str = "@Flash/0x08004000/48*001Kg";
```

The exposed map does not include the bootloader address space 0x08000000 - 0x08003fff since
that confuses fwupd, and it doesn't make sense to expose since the bootloader
can't update itself.

This directory contains a Makefile which should help you flash the bootloader into a
device. Please read the (security considerations)[../../#bootloader-security-considerations].


## Building

To make the bootloader you can use:

```bash
$ make
cargo build --release
   Compiling semver-parser v0.7.0
..
   Compiling dfu-bootloader v0.2.0 (/home/majopela/firmware/et/firmware-on-the-edge/firmware-examples/stm32f103/bootloader)
..
    Finished release [optimized + debuginfo] target(s) in 15.20s
arm-none-eabi-objcopy -O binary target/thumbv7m-none-eabi/release/dfu-bootloader dfu-bootloader.bin
```

You will get some warnings on `usb_vid_pid_is_for_private_testing_only` which is just a reminder
about setting your own USB VID/PID.

## Flashing

You can flash your device with the bootloader using `make flash` if you have an STLINK connected via usb,
connected to your board, and the board is powered on.

```bash
$ make flash

st-flash write dfu-bootloader.bin 0x8000000
st-flash 1.7.0
2023-03-17T16:22:07 INFO usb.c: Unable to match requested speed 1800 kHz, using 1000 kHz
2023-03-17T16:22:07 INFO common.c: F1xx Medium-density: 20 KiB SRAM, 128 KiB flash in at least 1 KiB pages.
file dfu-bootloader.bin md5 checksum: 387bbc85dd9a08288afe4f345ad9cd, stlink checksum: 0x00104666
2023-03-17T16:22:07 INFO common.c: Attempting to write 10056 (0x2748) bytes to stm32 address: 134217728 (0x8000000)
2023-03-17T16:22:07 INFO common.c: Flash page at addr: 0x08000000 erased
2023-03-17T16:22:07 INFO common.c: Flash page at addr: 0x08000400 erased
2023-03-17T16:22:07 INFO common.c: Flash page at addr: 0x08000800 erased
2023-03-17T16:22:07 INFO common.c: Flash page at addr: 0x08000c00 erased
2023-03-17T16:22:08 INFO common.c: Flash page at addr: 0x08001000 erased
2023-03-17T16:22:08 INFO common.c: Flash page at addr: 0x08001400 erased
2023-03-17T16:22:08 INFO common.c: Flash page at addr: 0x08001800 erased
2023-03-17T16:22:08 INFO common.c: Flash page at addr: 0x08001c00 erased
2023-03-17T16:22:08 INFO common.c: Flash page at addr: 0x08002000 erased
2023-03-17T16:22:08 INFO common.c: Flash page at addr: 0x08002400 erased
2023-03-17T16:22:08 INFO common.c: Finished erasing 10 pages of 1024 (0x400) bytes
2023-03-17T16:22:08 INFO common.c: Starting Flash write for VL/F0/F3/F1_XL
2023-03-17T16:22:08 INFO flash_loader.c: Successfully loaded flash loader in sram
2023-03-17T16:22:08 INFO flash_loader.c: Clear DFSR
 10/ 10 pages written
2023-03-17T16:22:09 INFO common.c: Starting verification of write complete
2023-03-17T16:22:09 INFO common.c: Flash written and verified! jolly good!
```

The bootloader is located at 0x08000000, and uses 16KB. This means that the application
should be compiled to run at 0x08004000 (see the memory.x linkerscript in the application
and bootloader directories).


Once flashed, you can see the device connecting via USB on the dmesg output:

```
[  109.423532] usb 2-2.12: new full-speed USB device number 7 using xhci_hcd
[  109.554068] usb 2-2.12: New USB device found, idVendor=2b23, idProduct=e011, bcdDevice= 0.01
[  109.554094] usb 2-2.12: New USB device strings: Mfr=1, Product=2, SerialNumber=3
[  109.554098] usb 2-2.12: Product: DFU Bootloader for STM32F103C8
[  109.554100] usb 2-2.12: Manufacturer: Red Hat
[  109.554103] usb 2-2.12: SerialNumber: 23934513
```