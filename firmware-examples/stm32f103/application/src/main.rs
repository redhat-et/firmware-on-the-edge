//! CDC-ACM serial port example using polling in a busy loop.
#![no_std]
#![no_main]

use panic_halt as _;

use core::str;
//use cortex_m::asm::delay;
use cortex_m_rt::entry;

use stm32f1xx_hal::usb::{Peripheral, UsbBus};
use stm32f1xx_hal::{prelude::*, stm32};
use usb_device::prelude::*;
use usbd_dfu_rt::{DfuRuntimeClass, DfuRuntimeOps};
use usbd_serial::SerialPort;

pub struct DFUBootloader;

const KEY_STAY_IN_BOOT: u32 = 0xb0d42b89;

impl DfuRuntimeOps for DFUBootloader {
    const DETACH_TIMEOUT_MS: u16 = 5000;
    const CAN_UPLOAD: bool = false;
    const WILL_DETACH: bool = true;

    fn detach(&mut self) {
        cortex_m::interrupt::disable();

        let cortex = unsafe { cortex_m::Peripherals::steal() };

        let p = 0x2000_0000 as *mut u32;
        unsafe { p.write_volatile(KEY_STAY_IN_BOOT) };

        cortex_m::asm::dsb();
        unsafe {
            // System reset request
            cortex.SCB.aircr.modify(|v| 0x05FA_0004 | (v & 0x700));
        }
        cortex_m::asm::dsb();
        loop {}
    }
}

#[entry]
fn main() -> ! {
    let dp = stm32::Peripherals::take().unwrap();

    let mut flash = dp.FLASH.constrain();
    let rcc = dp.RCC.constrain();

    let _clocks = rcc
        .cfgr
        .use_hse(8.MHz())
        .sysclk(48.MHz())
        .pclk1(24.MHz())
        .freeze(&mut flash.acr);

    //assert!(clocks.usbclk_valid());

    // Configure the on-board LED (PC13, green)
    let mut gpioc = dp.GPIOC.split();
    let mut board_led = gpioc.pc13.into_push_pull_output(&mut gpioc.crh);
    board_led.set_low(); // Turn on

    let mut gpioa = dp.GPIOA.split();
    let mut gpiob = dp.GPIOB.split();

    // BluePill board has a pull-up resistor on the D+ line.
    // Pull the D+ pin down to send a RESET condition to the USB bus.
    // This forced reset is needed only for development, without it host
    // will not reset your device when you upload new firmware.
    let mut usb_dp = gpioa.pa12.into_push_pull_output(&mut gpioa.crh);
    usb_dp.set_low();
    cortex_m::asm::delay(1024 * 10);

    let usb = Peripheral {
        usb: dp.USB,
        pin_dm: gpioa.pa11,
        pin_dp: usb_dp.into_floating_input(&mut gpioa.crh),
    };
    let usb_bus = UsbBus::new(usb);

    let mut serial = SerialPort::new(&usb_bus);
    let mut dfu = DfuRuntimeClass::new(&usb_bus, DFUBootloader);

    let mut usb_dev = UsbDeviceBuilder::new(&usb_bus, UsbVidPid(0x2b23, 0xe011))
        .manufacturer("Red Hat Inc.")
        .product("A Custom Peripheral")
        .serial_number(get_serial_str())
        .device_release(0x0202) // This is 2.01, you can use a version under the xml declared version
        // for development purposes (allows flashing many times as the version
        // will always be "updateable")
        .self_powered(false)
        .max_power(250)
        .max_packet_size_0(64)
        .build();

    let mut leds = [
        gpiob.pb8.into_push_pull_output(&mut gpiob.crh).erase(), // erase abstracts the type
        gpiob.pb6.into_push_pull_output(&mut gpiob.crl).erase(),
        gpiob.pb5.into_push_pull_output(&mut gpiob.crl).erase(),
        gpioa.pa10.into_push_pull_output(&mut gpioa.crh).erase(),
        gpiob.pb15.into_push_pull_output(&mut gpiob.crh).erase(),
    ];

    for led in leds.iter_mut() {
        led.set_low();
    }

    loop {
        if !usb_dev.poll(&mut [&mut serial, &mut dfu]) {
            // we need something that will tick every millisecond
            //delay(clocks.sysclk().0 / 1000);
            dfu.tick(1);
            continue;
        }

        let mut buf = [0u8; 64];

        match serial.read(&mut buf) {
            Ok(count) if count > 0 => {
                if buf[0] < b'0' || buf[0] > b'5' {
                    board_led.set_low(); // Turn on
                    continue;
                }

                let v = buf[0] - b'0' as u8;

                for (i, led) in leds.iter_mut().enumerate() {
                    if i >= v as usize {
                        led.set_low();
                    } else {
                        led.set_high();
                    }
                }
                board_led.set_high(); // Turn off
            }
            _ => {}
        }
    }
}

/// Returns device serial number as hex string slice.
fn get_serial_str() -> &'static str {
    static mut SERIAL: [u8; 8] = [b' '; 8];
    let serial = unsafe { SERIAL.as_mut() };

    fn hex(v: u8) -> u8 {
        match v {
            0..=9 => v + b'0',
            0xa..=0xf => v - 0xa + b'a',
            _ => b' ',
        }
    }

    let sn = read_serial();

    for (i, d) in serial.iter_mut().enumerate() {
        *d = hex(((sn >> (i * 4)) & 0xf) as u8)
    }

    unsafe { str::from_utf8_unchecked(serial) }
}

// Reads the serial number from the serial number field in the OTP
// memory.
// This is a 32-bit number that is unique to each device and
// is stored in the OTP memory.

fn read_serial() -> u32 {
    let u_id0 = 0x1FFF_F7E8 as *const u32;
    let u_id1 = 0x1FFF_F7EC as *const u32;

    unsafe { u_id0.read().wrapping_add(u_id1.read()) }
}
