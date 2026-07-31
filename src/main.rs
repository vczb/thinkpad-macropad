use std::process::Command;

use evdev::{Device, EventSummary, KeyCode, LedCode};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut device = Device::open("/dev/input/event3")?;

    let leds = device.get_led_state()?;

    let mut macro_mode = !leds.contains(LedCode::LED_NUML);

    loop {
        for event in device.fetch_events()? {
            if let EventSummary::Key(_, key, value) = event.destructure() {
                if key == KeyCode::KEY_NUMLOCK && value == 0 {
                    macro_mode = !macro_mode;
                    continue;
                }

                // println!("Key: {:?}, Value: {}, Macro {}", key, value, macro_mode);

                if macro_mode && value == 1 {
                    match key {
                        KeyCode::KEY_KP0 => {
                            Command::new("ulauncher-toggle").spawn()?;
                        }
                        KeyCode::KEY_KP1 => {
                            Command::new("firefox").spawn()?;
                        }
                        KeyCode::KEY_KP2 => {
                            Command::new("gnome-text-editor").spawn()?;
                        }
                        // KeyCode::KEY_KP3 => {
                        //     Command::new("xdg-open ~").spawn()?;
                        // }
                        _ => {}
                    }
                }
            }
        }
    }
}
