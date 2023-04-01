# STM32F411CEU6 based peripheral example

For environment setup see [this readme](../#environment-setup).

## Hardware

Recommended hardware to use the examples here is:
 * A design based on the stm32F411CEU6, you can use a board like the
   [blackpill](https://stm32-base.org/boards/STM32F411CEU6-WeAct-Black-Pill-V2.0.html) for a quick start.

## VID:PID details

Both bootloader and application share the same set of VID/PID to avoid the need of
a quirk file declaring a CounterpartGuid.

## Bootloader and Application

### Bootloader

The [bootloader](./bootloader/) directory contains the source code and necessary resources for
building the DFU bootloader for your USB-based peripheral device. The bootloader allows firmware
updates on the device using the DFU (Device Firmware Upgrade) protocol.

### Application

The [application](./application/) directory contains the source code for an example that
exposes a VCOM port to the linux host (found as /dev/ttyACMxxx) as well as the DFU application
runtime to let fwupd push the application back into bootloader mode when an update needs
to be performed.
