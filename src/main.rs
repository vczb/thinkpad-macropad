use std::process::Command;

use evdev::{Device, EventSummary, KeyCode};
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut device = Device::open("/dev/input/event3")?;

    let mut macro_mode = false;
    loop {
        for event in device.fetch_events()? {
            if let EventSummary::Key(_, key, value) = event.destructure() {
                match key {
                    KeyCode::KEY_NUMLOCK => {
                        macro_mode = value == 0;
                    }
                    _ => {}
                }
                if macro_mode {
                    match key {
                        KeyCode::KEY_KP0 => {
                            println!("Key: {:?}, Value: {}", key, value);
                            // Command::new("ulauncher-toggle").spawn()?;
                        }
                        KeyCode::KEY_KP1 => {
                            Command::new("firefox").spawn()?;
                        }
                        _ => {}
                    }
                }
            }
        }
    }
}
