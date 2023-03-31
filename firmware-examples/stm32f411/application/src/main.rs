//! CDC-ACM serial port example using polling in a busy loop.
#![no_std]
#![no_main]

use panic_halt as _;

#[rtic::app(device = stm32f4xx_hal::pac, peripherals = true)]
mod app {

    use core::str;
    use stm32f4xx_hal::{
        gpio,
        gpio::{Input, Output, PushPull},
        otg_fs::{UsbBus, UsbBusType, USB},
        pac,
        prelude::*,
        timer,
    };

    use usb_device::{class_prelude::*, prelude::*};

    use usbd_dfu_rt::{DfuRuntimeClass, DfuRuntimeOps};
    use usbd_serial::SerialPort;

    use arrform::{arrform, ArrForm};

    // Resources shared between tasks
    #[shared]
    struct Shared {
        timer: timer::CounterMs<pac::TIM2>,
        usb_dev: UsbDevice<'static, UsbBusType>,
        serial: SerialPort<'static, UsbBusType>,
        dfu: DfuRuntimeClass<DFUBootloader>,
    }

    // Local resources to specific tasks (cannot be shared)
    #[local]
    struct Local {
        button: gpio::PA0<Input>,
        led: gpio::PC13<Output<PushPull>>,
        delayval: u32,
    }

    #[init]
    fn init(ctx: init::Context) -> (Shared, Local, init::Monotonics) {
        static mut USB_BUS: Option<UsbBusAllocator<stm32f4xx_hal::otg_fs::UsbBus<USB>>,
        > = None;
        static mut EP_MEMORY: [u32; 1024] = [0; 1024];

        let dp = ctx.device;
        let rcc = dp.RCC.constrain();
        let clocks = rcc
            .cfgr
            .use_hse(25.MHz())
            .sysclk(48.MHz())
            .require_pll48clk()
            .freeze();

        // Configure the on-board LED (PC13, blue)
        let gpioc = dp.GPIOC.split();
        let gpioa = dp.GPIOA.split();
        let mut led = gpioc.pc13.into_push_pull_output();
        let button = gpioa.pa0.into_pull_up_input();

        let mut timer = dp.TIM2.counter_ms(&clocks);
        timer.start(100.millis()).unwrap();
        // Set up to generate interrupt when timer expires
        timer.listen(timer::Event::Update);

        // BlackPill board has a pull-up resistor on the D+ line.
        // Pull the D+ pin down to send a RESET condition to the USB bus.
        // This forced reset is needed only for development, without it host
        // will not reset your device when you upload new firmware.
        let mut usb_dp = gpioa.pa12.into_push_pull_output();
        usb_dp.set_low();
        cortex_m::asm::delay(1024 * 10);

        let usb_periph = USB {
            usb_global: dp.OTG_FS_GLOBAL,
            usb_device: dp.OTG_FS_DEVICE,
            usb_pwrclk: dp.OTG_FS_PWRCLK,
            hclk: clocks.hclk(),
            pin_dm: gpioa.pa11.into_alternate(),
            pin_dp: usb_dp.into_alternate(),
        };

        unsafe {
            USB_BUS = Some(UsbBus::new(usb_periph, &mut EP_MEMORY));
        }

        let serial = SerialPort::new(unsafe { USB_BUS.as_ref().unwrap() });
        let dfu = DfuRuntimeClass::new(unsafe { USB_BUS.as_ref().unwrap() }, DFUBootloader);

        let usb_dev = UsbDeviceBuilder::new(
            unsafe { USB_BUS.as_ref().unwrap() },
            UsbVidPid(0x2b23, 0xe012),
        )
        .manufacturer("Red Hat Inc.")
        .product("A Custom Peripheral")
        .serial_number(get_serial_str())
        .device_release(0x0202) // This is 2.02, you can use a version under the xml declared version
        // for development purposes (allows flashing many times as the version
        // will always be "updateable")
        .self_powered(false)
        .max_power(250)
        .max_packet_size_0(64)
        .build();

        led.set_high();

        let mut vtor: u32 = 0;

        /*    unsafe {
            cortex_m::peripheral::Peripherals::steal().SCB.vtor.write(0x8000);
            vtor = cortex_m::peripheral::Peripherals::steal().SCB.vtor.read();

        }
        */

        (
            Shared {
                timer,
                usb_dev,
                serial,
                dfu,
            },
            Local {
                button,
                led,
                delayval: 2000_u32,
            },
            // Move the monotonic timer to the RTIC run-time, this enables
            // scheduling
            init::Monotonics(),
        )
    }

    #[task(binds = OTG_FS, shared = [usb_dev, serial, dfu])]
    fn usb_task(mut cx: usb_task::Context) {
        let usb_dev = &mut cx.shared.usb_dev;
        let serial = &mut cx.shared.serial;
        let dfu = &mut cx.shared.dfu;

        (usb_dev, serial, dfu).lock(|usb_dev, serial, dfu| {
            if !usb_dev.poll(&mut [serial, dfu]) {
                return;
            }
            let mut buf = [0u8; 64];

            match serial.read(&mut buf) {
                Ok(count) if count > 0 => {
                    let af = arrform!(64, "Received {} bytes: {:02x?}\r\n", count, &buf[..count]);
                    serial.write(af.as_bytes()).ok();
                }
                _ => {}
            }
        });
    }

    #[task(binds = TIM2, shared=[timer, dfu], local=[led])]
    fn timer_expired(mut ctx: timer_expired::Context) {
        // When Timer Interrupt Happens Two Things Need to be Done
        // 1) Toggle the LED
        // 2) Clear Timer Pending Interrupt

        ctx.shared.dfu.lock(|dfu| dfu.tick(100));
        ctx.shared
            .timer
            .lock(|tim| tim.clear_interrupt(timer::Event::Update));
        ctx.local.led.toggle();
    }

    // Background task, runs whenever no other tasks are running
    #[idle]
    fn idle(_: idle::Context) -> ! {
        loop {
            // Go to sleep
            cortex_m::asm::wfi()
        }
    }

    pub struct DFUBootloader;

    const KEY_STAY_IN_BOOT: u32 = 0xb0d42b89;

    impl DfuRuntimeOps for DFUBootloader {
        const DETACH_TIMEOUT_MS: u16 = 500;
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

    /// Return device serial based on U_ID registers.
    fn read_serial() -> u32 {
        let u_id0 = 0x1FFF_7A10 as *const u32;
        let u_id1 = 0x1FFF_7A14 as *const u32;

        unsafe { u_id0.read().wrapping_add(u_id1.read()) }
    }
}
