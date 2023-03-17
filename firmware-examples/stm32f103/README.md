# STM32F103C8 based peripheral example

## Environment

For development environment in this case, we recommend a Fedora machine with the following
packages installed:

```bash
$ dnf install -y stlink openocd arm-none-eabi-binutils-cs

# install the community version of rust, and the ARM thumbv7m-none-eaby target
$ curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
$ source "$HOME/.cargo/env"
$ rustup target add thumbv7m-none-eabi

```

## Hardware

Recommended hardware to use the examples here is:
 * [STLINKv3 probe](https://www.st.com/en/development-tools/stlink-v3set.html), this is a jtag/swd
   tool, to help you program and debug embedded designs.

 * A design based on the stm32f103c8, you can use a board like the
   [bluepill](https://stm32-base.org/boards/STM32F103C8T6-Blue-Pill.html) for a quick start.


## VID:PID details

Both bootloader and application share the same set of VID/PID to avoid the need of
a quirk file declaring a CounterpartGuid.

## Bootloader and Application

### Bootloader

The [./bootloader](bootloader) directory contains the source code and necessary resources for
building the DFU bootloader for your USB-based peripheral device. The bootloader allows firmware
updates on the device using the DFU (Device Firmware Upgrade) protocol.

### Application

The [application](./application/) directory contains the source code for an example that
exposes a VCOM port to the linux host (found as /dev/ttyACMxxx) as well as the DFU application
runtime to let fwupd push the application back into bootloader mode when an update needs
to be performed.
