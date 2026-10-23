//! Loading and saving Clip5's slots and settings to `clip5.json`.

use std::fs;
use std::io;
use std::path::{Path, PathBuf};

use serde::{Deserialize, Serialize};

pub const SLOT_COUNT: usize = 5;

pub type Slots = [String; SLOT_COUNT];

const FILE_NAME: &str = "clip5.json";
const TEMP_FILE_NAME: &str = "clip5.json.tmp";
const CORRUPT_FILE_NAME: &str = "clip5.corrupt.json";

/// Everything Clip5 remembers between launches.
#[derive(Clone, Debug, PartialEq)]
pub struct Config {
    pub slots: Slots,
    pub always_on_top: bool,
    /// `None` until the window has been shown once.
    pub window: Option<WindowRect>,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            slots: Slots::default(),
            always_on_top: true,
            window: None,
        }
    }
}

/// The window's outer top-left position and inner size, in points.
#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub struct WindowRect {
    pub x: f32,
    pub y: f32,
    pub width: f32,
    pub height: f32,
}

/// The JSON layout. Every field has a default so older or hand-edited files still load.
#[derive(Serialize, Deserialize)]
struct ConfigFile {
    #[serde(default)]
    slots: Vec<String>,
    #[serde(default = "default_always_on_top")]
    always_on_top: bool,
    #[serde(default)]
    window: Option<WindowRect>,
}

fn default_always_on_top() -> bool {
    true
}

/// The per-user folder for Clip5's files: `%APPDATA%\Clip5` on Windows,
/// `~/Library/Application Support/Clip5` on macOS.
pub fn data_dir() -> Option<PathBuf> {
    dirs::config_dir().map(|dir| dir.join("Clip5"))
}

/// Loads the config from `dir`. A missing file gives the defaults. An unreadable file is
/// renamed to `clip5.corrupt.json` so the next save doesn't overwrite it.
pub fn load(dir: &Path) -> Config {
    let path = dir.join(FILE_NAME);

    let text = match fs::read_to_string(&path) {
        Ok(text) => text,
        Err(err) if err.kind() == io::ErrorKind::NotFound => return Config::default(),
        Err(_) => {
            set_aside(&path);
            return Config::default();
        }
    };

    // Editors such as Notepad may add a byte-order mark, which serde_json rejects.
    match serde_json::from_str::<ConfigFile>(text.trim_start_matches('\u{feff}')) {
        Ok(file) => {
            let mut slots = Slots::default();
            for (slot, text) in slots.iter_mut().zip(file.slots) {
                *slot = text;
            }
            Config {
                slots,
                always_on_top: file.always_on_top,
                window: file.window,
            }
        }
        Err(_) => {
            set_aside(&path);
            Config::default()
        }
    }
}

/// Saves the config to `dir`, writing a temporary file first so a crash mid-write
/// can't leave a half-written `clip5.json`.
pub fn save(dir: &Path, config: &Config) -> io::Result<()> {
    fs::create_dir_all(dir)?;
    let json = serde_json::to_string_pretty(&ConfigFile {
        slots: config.slots.to_vec(),
        always_on_top: config.always_on_top,
        window: config.window,
    })?;
    let temp = dir.join(TEMP_FILE_NAME);
    fs::write(&temp, json)?;
    fs::rename(&temp, dir.join(FILE_NAME))
}

fn set_aside(path: &Path) {
    let _ = fs::rename(path, path.with_file_name(CORRUPT_FILE_NAME));
}

#[cfg(test)]
mod tests {
    use super::*;

    /// A fresh, empty folder under the system temp directory.
    fn temp_dir(name: &str) -> PathBuf {
        let dir = std::env::temp_dir().join(format!("clip5-test-{name}-{}", std::process::id()));
        let _ = fs::remove_dir_all(&dir);
        dir
    }

    #[test]
    fn save_then_load_round_trips() {
        let dir = temp_dir("round-trip");
        let config = Config {
            slots: ["one", "two\nlines", "", "fünf ☕", "five"].map(String::from),
            always_on_top: false,
            window: Some(WindowRect {
                x: 100.0,
                y: 50.0,
                width: 420.0,
                height: 300.0,
            }),
        };
        save(&dir, &config).unwrap();
        assert_eq!(load(&dir), config);
        assert!(!dir.join(TEMP_FILE_NAME).exists());
        fs::remove_dir_all(&dir).unwrap();
    }

    #[test]
    fn missing_file_gives_defaults() {
        let dir = temp_dir("missing");
        assert_eq!(load(&dir), Config::default());
    }

    #[test]
    fn corrupt_file_is_set_aside() {
        let dir = temp_dir("corrupt");
        fs::create_dir_all(&dir).unwrap();
        fs::write(dir.join(FILE_NAME), "not json").unwrap();

        assert_eq!(load(&dir), Config::default());
        assert!(!dir.join(FILE_NAME).exists());
        assert_eq!(
            fs::read_to_string(dir.join(CORRUPT_FILE_NAME)).unwrap(),
            "not json"
        );
        fs::remove_dir_all(&dir).unwrap();
    }

    #[test]
    fn byte_order_mark_is_tolerated() {
        let dir = temp_dir("bom");
        fs::create_dir_all(&dir).unwrap();
        fs::write(dir.join(FILE_NAME), "\u{feff}{\"slots\":[\"a\"]}").unwrap();
        assert_eq!(load(&dir).slots, ["a", "", "", "", ""].map(String::from));
        fs::remove_dir_all(&dir).unwrap();
    }

    #[test]
    fn missing_settings_use_defaults() {
        let dir = temp_dir("defaults");
        fs::create_dir_all(&dir).unwrap();
        fs::write(dir.join(FILE_NAME), r#"{"slots":["a"]}"#).unwrap();
        let config = load(&dir);
        assert!(config.always_on_top);
        assert_eq!(config.window, None);
        fs::remove_dir_all(&dir).unwrap();
    }

    #[test]
    fn wrong_slot_count_is_tolerated() {
        let dir = temp_dir("count");
        fs::create_dir_all(&dir).unwrap();
        fs::write(
            dir.join(FILE_NAME),
            r#"{"slots":["a","b","c","d","e","f","g"]}"#,
        )
        .unwrap();
        assert_eq!(
            load(&dir).slots,
            ["a", "b", "c", "d", "e"].map(String::from)
        );

        fs::write(dir.join(FILE_NAME), r#"{"slots":["a"]}"#).unwrap();
        assert_eq!(load(&dir).slots, ["a", "", "", "", ""].map(String::from));
        fs::remove_dir_all(&dir).unwrap();
    }
}
