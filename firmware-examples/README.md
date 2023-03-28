
# Firmware examples

This repository contains firmware update examples for USB-based peripherals on RHEL systems.

These examples utilize the [STM32F103](https://www.st.com/en/microcontrollers-microprocessors/stm32f103.html)
and the [STM32F411](https://www.st.com/en/microcontrollers-microprocessors/stm32f411.html)
microcontroller and the [Rust](https://www.rust-lang.org/) programming language. However, the
techniques demonstrated can be applied to other languages as well.

The examples consist of two folders: one for the bootloader and another for the peripheral firmware application.

## Bootloader and Update protocol

For USB devices, we recommend implementing the [DFU](https://www.usb.org/sites/default/files/DFU_1.1.pdf)
or DFUse protocols, as fwupd supports these protocols. Our examples utilize the DFUse version of the
protocol.

DFU defines two interfaces: a bootloader interface that can write, erase, or read (if enabled) the
application space, and a runtime interface that allows the host to request the application to jump
back to the bootloader.

![DFU Bootloader and APP](./dfu.svg)


The DFU protocol is implemented using the [usbd-dfu](https://github.com/vitalyvb/usbd-dfu) Rust
library, which is based on the [usb-device](https://github.com/rust-embedded-community/usb-device)
stack. We build the bootloaders using the [usbd-dfu-example](https://github.com/vitalyvb/usbd-dfu-example)
provided by the author.

## Application

The applications implement the DFU runtime interface, declaring the device as DFU-enabled and
accepting the DFU_DETACH and GET_STATUS commands.

We implement the DFU runtime using the [usbd-dfu-rt](https://github.com/jedrzejboczar/usbd-dfu-rt) Rust
library, which is based on the [usb-device](https://github.com/rust-embedded-community/usb-device) stack.

As part of the application firmware build process we provide examples of the fwupd xml metadata
as well as .cab file wrapping. Fwupd can consume the resulting cab file from a vendor directory
remote, i.e.:

`$ cat /etc/fwupd/remotes.d/vendor-directory.conf`
```ini
[fwupd Remote]
# this remote provides dynamically generated metadata shipped by the OS vendor and can
# be found in /usr/share/fwupd/remotes.d/vendor/firmware
Enabled=true
Title=Vendor (Automatic)
# using `Keyring=none` is required as directory remotes requires the local user to trust the person
# putting the files in the immutable local location.
Keyring=none
MetadataURI=file:///usr/share/fwupd/remotes.d/vendor/firmware
ApprovalRequired=false
```

## USB Considerations

A USB host (e.g., a RHEL device) identifies peripherals based on their location in the USB tree and
their VID/PID. Ensuring a unique VID/PID for each specific peripheral type is essential to:

* Avoid conflict with other peripheral drivers
* Facilitate fwupd's firmware-to-peripheral matching process

In development and highly controlled environments, you can choose non conflicting VID/PID
combination. For mass-produced and publicly sold peripherals,
[obtain a VID from usb.org](https://www.usb.org/getting-vendor-id)
or sublicense a PID from any registered vendor or your chip maker. This process is
similar for PCIe-connected devices.

## USB Considerations

A USB host (e.g., a RHEL device) identifies peripherals based on their location in the USB tree and
their VID/PID. VID and PID are 16bit numbers, VIDs are assigned by usb.org to vendors.

Ensuring a unique VID/PID for each specific peripheral type is essential to:

* Avoid conflicts with other peripheral drivers
* Facilitate fwupd's firmware-to-peripheral matching process.

In development and highly controlled environments, you can choose your VID/PID combination. For
mass-produced and publicly sold peripherals, [obtain a VID from usb.org](https://www.usb.org/getting-vendor-id)
or sublicense a PID from any registered vendor or your chip maker. This process is similar for
PCIe-connected devices.

## Bootloader Security Considerations

Consider the examples provided as a reference for creating your peripheral firmware
in a way that fwupd can handle updates and as a suitable starting point for using Rust.

As a peripheral builder, you may want to apply the following considerations, depending
on the environment or type of device you are building:

* Adding encryption (decryption at bootloader) to the firmware blob. While algorithms
    like XTEA aren't completely secure, they can help. AES is a better option if your
    device supports it.
* Adding signature/verification to the firmware blobs if your device has hardware support.
* Disabling JTAG access on boot.
* Disabling read access (we do this by default).
* Disabling JTAG read access on your device at programming time.

## Notes on BOOT ROM DFU Bootloaders

Some MCUs include their own DFU bootloader in ROM. This eliminates the need to write and
maintain your own bootloader. However, one disadvantage of using the ROM Bootloader is that
the VID:PID of the bootloader will be the chip manufacturer's pair, and your peripheral
will have two pairs:

* one in bootloader mode
* one in the final application.

You will need to indicate this via quirks to fwupd. It is not recommended, as fwupd
won't be able to recognize and reflash your device if anything goes wrong,
since that bootloader VID/PID pair would be shared across many peripherals
built on the same chip.

