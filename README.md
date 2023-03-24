# Firmware on the Edge

This project is focused on streamlining the integration and management of firmware
for custom or off-the-shelf peripherals in edge systems. 

We aim to simplify the development cycle by tying the lifecycle of RHEL Edge device
firmware with peripherals' firmware. 

Edge systems in many cases require interaction with the physical world to carry on their
application, i.e.:

* Medical devices
* Drones
* Industrial control systems
* Vehicles
* Agricultural control systems
* Smart cities
* Radar Systems
* Anything with human interfaces

Interaction with the world requires special peripherals, sometimes custom built by our
customers for specific purposes, sometimes off-the-shelf peripherals. Those peripherals,
generally built on MCUs, carry firmware.

# Embedding firmware in the operating system image

RHEL systems include the [fwupd](https://github.com/fwupd) daemon since Centos/RHEL 7.4 in 2017.
The fwupd project has been helping update our servers and laptop peripherals through the
[LVFS project](https://lvfs.org) since then. Vendors can publish firmware updates for
their devices through the LVFS CDN.

Fwupd can also source firmware from local directories, which perfectly matches our purpose; we can enable ImageBuilder and osbuild to allow that, see:
* [COMPOSER-1931](https://issues.redhat.com/browse/COMPOSER-1931) osbuild: support creating/enabling fwupd remotes.
* [COMPOSER-1932](https://issues.redhat.com/browse/COMPOSER-1932) osbuild: embedding firmware from container sources.
* [COMPOSER-1933](https://issues.redhat.com/browse/COMPOSER-1933) osbuild: embedding firmware from LVFS.
* [COMPOSER-1934](https://issues.redhat.com/browse/COMPOSER-1934) composer: implement firmware embedding in images.

One option today, and we build or demo based on that, is embedding firmware in
[rpms and repositories](https://copr.fedorainfracloud.org/coprs/g/redhat-et/firmware-on-the-edge/),
which we can then source and include in ImageBuilder. We explain this workflow,
but we don't consider it a final solution since creating and exposing repositories
is an unnecessary complexity in a world where container registries exist today; that
is the reason behind [COMPOSER-1934](https://issues.redhat.com/browse/COMPOSER-1934)
and [COMPOSER-1932](https://issues.redhat.com/browse/COMPOSER-1932).

## Missing capabilities and future ideas

Fwupd cannot perform auto-updates. It's capable of detecting know devices,
the firmware versions, informing via its api `fwupdmgr get-devices --json` about the status,
and then accepting requests to update, downgrade or reinstall.

In this repository, we provide an experiment that performs updates on startup:
[fwupd-auto-update](./fwupd-auto-update/).

A future idea is to have a daemon that will perform auto-updates; this idea is proposed
here: [RFE for fwupd auto updates](https://github.com/fwupd/fwupd/discussions/5641).


# Updating devices and building custom firmware.

`fwupd` supports a variety of protocols to update devices, including many standards.
When it's possible, the recommendation is to use a standard protocol, like DFU/DFUse
in the case of USB. If a standard protocol is impossible, please look at the
[fwupd documentation on writing custom plugins](https://lvfs.readthedocs.io/en/latest/custom-plugin.html).

Our examples in the (firmware-examples)[./firmware-examples] directory use 
the USB/DFU protocol, and the Rust language.
