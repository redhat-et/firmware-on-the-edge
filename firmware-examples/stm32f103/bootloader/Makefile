# Copyright (C) 2023 Miguel Angel Ajo <majopela@redhat.com>
#
# Licensed under the GNU General Public License Version 2
#
# This program is free software; you can redistribute it and/or modify
# it under the terms of the GNU General Public License as published by
# the Free Software Foundation; either version 2 of the License, or
# (at your option) any later version.
#
# This program is distributed in the hope that it will be useful,
# but WITHOUT ANY WARRANTY; without even the implied warranty of
# MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
# GNU General Public License for more details.
#
# You should have received a copy of the GNU General Public License
# along with this program; if not, write to the Free Software
# Foundation, Inc., 51 Franklin Street, Fifth Floor, Boston, MA 02110-1301 USA.
#

all: dfu-bootloader.bin

target/thumbv7m-none-eabi/release/dfu-bootloader: src/main.rs Cargo.toml memory.x
	cargo build --release

dfu-bootloader.bin: target/thumbv7m-none-eabi/release/dfu-bootloader 
	arm-none-eabi-objcopy -O binary target/thumbv7m-none-eabi/release/dfu-bootloader dfu-bootloader.bin

flash: dfu-bootloader.bin
	st-flash write dfu-bootloader.bin 0x8000000

openocd-b: flash
	openocd -c "set CPUTAPID 0x2ba01477" -f openocd.cfg

openocd: flash
	openocd -f openocd.cfg

clean:
	rm -rf target dfu-bootloader.bin


.PHONY: clean all flash openocd
