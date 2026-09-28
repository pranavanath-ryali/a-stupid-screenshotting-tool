use std::{env, process::Command};

use serde::Deserializer;
use serde_json::Value;

#[derive(Debug)]
pub enum Compositor {
    Mango,

    Unknown,
}
impl Compositor {
    pub fn detect() -> Self {
        let compositor = env::var("XDG_CURRENT_DESKTOP")
            .or_else(|_| env::var("DESKTOP_SESSION"))
            .unwrap_or_default()
            .to_lowercase();

        if compositor.eq("mango") {
            return Compositor::Mango;
        }

        Compositor::Unknown
    }
}

pub fn get_active_client_bounds(compositor: &Compositor) -> Option<(i32, i32, u32, u32)> {
    match compositor {
        Compositor::Mango => {
            let output = Command::new("mmsg")
                .arg("get")
                .arg("focusing-client")
                .output()
                .expect("failed to execute mmsg ipc call");

            let data: Value = serde_json::from_slice(&output.stdout).expect("failed to get data");

            let x_value = data.get("x").unwrap();
            let y_value = data.get("y").unwrap();
            let width_value = data.get("width").unwrap();
            let height_value = data.get("height").unwrap();

            let x: i32 = match x_value {
                Value::Number(number) => {
                    number.as_i64().and_then(|n| i32::try_from(n).ok()).unwrap()
                }
                _ => {
                    panic!("Failde to get field x")
                }
            };
            let y: i32 = match y_value {
                Value::Number(number) => {
                    number.as_i64().and_then(|n| i32::try_from(n).ok()).unwrap()
                }
                _ => {
                    panic!("Failde to get field y")
                }
            };
            let width: u32 = match width_value {
                Value::Number(number) => {
                    number.as_u64().and_then(|n| u32::try_from(n).ok()).unwrap()
                }
                _ => {
                    panic!("Failde to get field width")
                }
            };
            let height: u32 = match height_value {
                Value::Number(number) => {
                    number.as_u64().and_then(|n| u32::try_from(n).ok()).unwrap()
                }
                _ => {
                    panic!("Failde to get field height")
                }
            };

            Some((x, y, width, height))
        }
        Compositor::Unknown => None,
    }
}
