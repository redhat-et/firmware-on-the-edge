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

rpmbuild/SOURCES:
	mkdir -p ./rpmbuild/{BUILD,RPMS,SOURCES,SPECS,SRPMS}

rpmbuild/SOURCES/fwupd-auto-update.service: rpmbuild/SOURCES fwupd-auto-update.service
	cp fwupd-auto-update.service rpmbuild/SOURCES

rpm: rpmbuild/SOURCES/fwupd-auto-update.service fwupd-auto-update.spec
	rpmbuild -bb --define "_topdir $(CURDIR)/rpmbuild" fwupd-auto-update.spec

srpm: rpmbuild/SOURCES/fwupd-auto-update.service fwupd-auto-update.spec
	rpmbuild -bs --define "_topdir $(CURDIR)/rpmbuild" fwupd-auto-update.spec

copr: clean srpm
	copr-cli build @redhat-et/firmware-on-the-edge rpmbuild/SRPMS/fwupd-auto-update-*.src.rpm

all: srpm rpm

clean:
	rm -rf rpmbuild

