# Copyright (C) 2015 Richard Hughes <richard@hughsie.com>
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

TARGET		= stm32f103-example-vcp
VERSION		= 2.2

all: $(TARGET)-$(VERSION).cab

CAB_FILES=							\
	example.bin			\
	firmware.metainfo.xml

check: firmware.metainfo.xml
	appstream-util validate-relax $<

target/thumbv7m-none-eabi/release/example-stm32f103c8: src/main.rs Cargo.toml memory.x
	cargo build --release

example.bin: target/thumbv7m-none-eabi/release/example-stm32f103c8
	arm-none-eabi-objcopy -Obinary target/thumbv7m-none-eabi/release/example-stm32f103c8 example.bin

example.hex:target/thumbv7m-none-eabi/release/example-stm32f103c8
	arm-none-eabi-objcopy -Oihex target/thumbv7m-none-eabi/release/example-stm32f103c8 example.hex

rpmbuild/SOURCES:
	mkdir -p ./rpmbuild/{BUILD,RPMS,SOURCES,SPECS,SRPMS}

rpmbuild/SOURCES/$(TARGET)-${VERSION}.cab: rpmbuild/SOURCES $(TARGET)-$(VERSION).cab
	cp $(TARGET)-${VERSION}.cab rpmbuild/SOURCES

rpm: rpmbuild/SOURCES/$(TARGET)-${VERSION}.cab stm32f103-example-firmware.spec
	rpmbuild -bb --define "_topdir $(CURDIR)/rpmbuild" stm32f103-example-firmware.spec

srpm: rpmbuild/SOURCES/$(TARGET)-${VERSION}.cab stm32f103-example-firmware.spec
	rpmbuild -bs --define "_topdir $(CURDIR)/rpmbuild" stm32f103-example-firmware.spec

$(TARGET)-${VERSION}.cab: $(CAB_FILES) check
	gcab --create --nopath $@ $(CAB_FILES)

copr: clean srpm
	copr-cli build @redhat-et/firmware-on-the-edge rpmbuild/SRPMS/stm32f103*.src.rpm

all: $(TARGET)-${VERSION}.cab

clean:
	rm -rf target example.hex example.bin $(TARGET)-${VERSION}.cab rpmbuild

