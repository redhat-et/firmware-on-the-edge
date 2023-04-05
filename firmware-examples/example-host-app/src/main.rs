
use std::time::Duration;
use std::str;



const RADAR_PORT: &str = "/dev/radar";
const DISPLAY_PORT: &str = "/dev/display";
const BAUD_RATE: u32 = 115200;

fn main() {
    println!("Radar based application");
    let mut radar = serialport::new(RADAR_PORT, BAUD_RATE)
        .timeout(Duration::from_millis(1000))
        .open().unwrap();

    let mut display = serialport::new(DISPLAY_PORT, BAUD_RATE)
        .timeout(Duration::from_millis(1000))
        .open().unwrap();

    let mut buf = [0; 128];
    let mut too_far_count = 0;

    loop {
        // read will fail on timeout or if port is closed
        let bytes_read = radar.read(&mut buf).unwrap();

        // convert to string and split
        let reading = str::from_utf8(&buf[0..bytes_read]).unwrap();
        let parts = reading.split("\n");

        for part in parts {
            let trimmed = part.trim();
            if trimmed.len() > 0 {
                println!("{}", trimmed);
                if let Ok(cm) = trimmed.parse::<u32>() {
                    if cm > 50 {
                        too_far_count += 1;
                        println!("tf {}", too_far_count);
                        if too_far_count > 10 {
                            display_level(&mut display, 0, cm);
                        } else {
                            continue
                        }
                    } else {
                        too_far_count = 0;
                        if cm < 50 {
                            let level = ((50 - cm) + 9) / 10; // 0..5 level for 0..50cm
                            display_level(&mut display, level, cm);
                        } else {
                            display_level(&mut display, 0, cm);
                        }
                    }
                }
            }
        }
    }
}

fn display_level(display: &mut Box<dyn serialport::SerialPort>, level: u32, cm: u32) {
    display.write(format!("{}", level).as_bytes()).unwrap();

    println!("Radar echo: {}cm, display level {}", cm, level);
}
