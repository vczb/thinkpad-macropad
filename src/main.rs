use std::process::{Command, Stdio};

use evdev::{Device, EventSummary, KeyCode, LedCode};

fn launch(command: &str, args: Option<&[&str]>) -> std::io::Result<()> {
    Command::new(command)
        .args(args.unwrap_or(&[]))
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .spawn()?;

    Ok(())
}

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
                            launch("ulauncher-toggle", None)?;
                        }
                        KeyCode::KEY_KP1 => {
                            launch("firefox", None)?;
                        }
                        KeyCode::KEY_KP2 => {
                            launch("gnome-text-editor", None)?;
                        }
                        KeyCode::KEY_KP3 => {
                            let home = std::env::var("HOME")?;

                            launch("xdg-open", Some(&[&home]))?;
                        }
                        _ => {}
                    }
                }
            }
        }
    }
}
