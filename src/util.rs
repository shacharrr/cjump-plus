use mki::Keyboard;
use std::collections::HashMap;
use serde::{Serialize, Deserialize};


#[derive(Deserialize, Serialize)]
pub struct TomlConVar {
    pub dark_mode: String,
    pub config: TomlConfig,
}

impl TomlConVar {
    pub const fn new() -> TomlConVar {
        TomlConVar { 
            dark_mode: String::new(),
            config: TomlConfig {
                jump_bind: String::new(),
                duck_bind: String::new(),
                pause_bind: String::new(),
                delay: String::new(),
            }
        }
    }

    pub fn convert_to_convar(&mut self) -> ConVar {
        ConVar {
            dark_mode: self.dark_mode.parse().unwrap(),
            config: Config {
                jump_bind: self.config.jump_bind.clone(),
                duck_bind: self.config.duck_bind.clone(),
                pause_bind: self.config.pause_bind.clone(),
                delay: self.config.delay.parse().unwrap(),
            }
        }
    }
}

#[derive(Deserialize, Serialize)]
pub struct TomlConfig {
    pub jump_bind: String,
    pub duck_bind: String,
    pub pause_bind: String,
    pub delay: String,
}

pub struct ConVar {
    pub dark_mode: u8,
    pub config: Config,
}

impl ConVar {
    pub const fn new() -> ConVar {
        ConVar { 
            dark_mode: 1,
            config: Config {
                jump_bind: String::new(),
                duck_bind: String::new(),
                pause_bind: String::new(),
                delay: 850,
            }
        }
    }

    pub fn convert_to_toml(&mut self) -> TomlConVar {
        TomlConVar {
            dark_mode: self.dark_mode.to_string(),
            config: TomlConfig {
                jump_bind: self.config.jump_bind.clone(),
                duck_bind: self.config.duck_bind.clone(),
                pause_bind: self.config.pause_bind.clone(),
                delay: self.config.delay.to_string(),
            }
        }
    }
}

pub struct Config {
    pub jump_bind: String,
    pub duck_bind: String,
    pub pause_bind: String,
    pub delay: u64,
}

pub fn create_jump_hash() -> HashMap<String, Keyboard> {
    let mut jump_vk = HashMap::new();
    jump_vk.insert(
        "Space".to_string(),
        Keyboard::Space,
    );
    jump_vk.insert(
        "V".to_string(),
        Keyboard::V,
    );
    jump_vk.insert(
        "B".to_string(),
        Keyboard::B,
    );
    jump_vk.insert(
        "N".to_string(),
        Keyboard::N,
    );
    jump_vk
}

pub fn create_duck_hash() -> HashMap<String, Keyboard> {
    let mut jump_vk = HashMap::new();
    jump_vk.insert(
        "Left Control".to_string(),
        Keyboard::LeftControl,
    );
    jump_vk.insert(
        "C".to_string(),
        Keyboard::C,
    );
    jump_vk
}

pub fn create_pause_hash() -> HashMap<String, Keyboard> {
    let mut pause_vk = HashMap::new();
    pause_vk.insert(
        "Enter".to_string(),
        Keyboard::Enter,
    );
    pause_vk.insert(
        "Tab".to_string(),
        Keyboard::Tab,
    );
    pause_vk.insert(
        "Backspace".to_string(),
        Keyboard::BackSpace,
    );
    pause_vk.insert(
        "Left Alt".to_string(),
        Keyboard::LeftAlt,
    );
    pause_vk.insert(
        "F1".to_string(),
        Keyboard::F1,
    );
    pause_vk.insert(
        "F2".to_string(),
        Keyboard::F2,
    );
    pause_vk.insert(
        "F3".to_string(),
        Keyboard::F3,
    );
    pause_vk.insert(
        "F4".to_string(),
        Keyboard::F4,
    );
    pause_vk.insert(
        "F5".to_string(),
        Keyboard::F5,
    );
    pause_vk.insert(
        "F6".to_string(),
        Keyboard::F6,
    );
    pause_vk.insert(
        "F7".to_string(),
        Keyboard::F7,
    );
    pause_vk.insert(
        "F8".to_string(),
        Keyboard::F8,
    );
    pause_vk.insert(
        "F9".to_string(),
        Keyboard::F9,
    );
    pause_vk.insert(
        "F10".to_string(),
        Keyboard::F10,
    );
    pause_vk.insert(
        "F11".to_string(),
        Keyboard::F11,
    );
    pause_vk.insert(
        "F12".to_string(),
        Keyboard::F12,
    );
    pause_vk
}