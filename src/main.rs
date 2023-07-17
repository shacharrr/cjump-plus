#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release


use mki::Keyboard;
use std::{thread, io::Write};
use std::time::Duration;
use serde::{Serialize, Deserialize};
use std::fs;
use std::collections::HashMap;

use eframe::egui;
use egui::{menu, Visuals};


const CFG_PATH: &str = "./cfg/cjump.toml";
// Shity way should improve
static mut FLOOP: bool = true;
static mut PAUSED: bool = false;

static mut TOML_CONVAR: TomlConVar = TomlConVar::new();
static mut CONVAR: ConVar =  ConVar::new();

#[derive(Deserialize, Serialize)]
struct TomlConVar {
    dark_mode: String,
    config: TomlConfig,
}

impl TomlConVar {
    const fn new() -> TomlConVar {
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

    fn convert_to_convar(&mut self) -> ConVar {
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
struct TomlConfig {
    jump_bind: String,
    duck_bind: String,
    pause_bind: String,
    delay: String,
}

struct ConVar {
    dark_mode: u8,
    config: Config,
}

impl ConVar {
    const fn new() -> ConVar {
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

    fn convert_to_toml(&mut self) -> TomlConVar {
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

struct Config {
    jump_bind: String,
    duck_bind: String,
    pause_bind: String,
    delay: u64,
}


fn main() -> Result<(), eframe::Error> {
    match std::fs::metadata(CFG_PATH) {
        Ok(_) => {
            let contents = fs::read_to_string(CFG_PATH)
            .expect("Should have been able to read the file");
            
            unsafe {
                TOML_CONVAR = toml::from_str(contents.as_str()).unwrap();
                CONVAR = TOML_CONVAR.convert_to_convar();
            }
        },
        Err(_) => {
            let path = std::path::Path::new(CFG_PATH);
            let prefix = path.parent().unwrap();
            std::fs::create_dir_all(prefix).unwrap();
            let mut f = std::fs::File::create(path).unwrap();

            let default_toml = r#"dark_mode = '1'

[config]
jump_bind = 'Space'
duck_bind = 'Left Control'
pause_bind = 'Enter'
delay = '850'
            "#;

            let _ = f.write_all(default_toml.as_bytes());
            
            unsafe {
                TOML_CONVAR = toml::from_str(default_toml).unwrap();
                CONVAR = TOML_CONVAR.convert_to_convar();
            };
        },
    }

    thread::spawn(|| {
        let jump_vk = create_jump_hash();
        let duck_vk = create_duck_hash();
        let pause_vk = create_pause_hash();
        loop {
            let mut spfloop = true;
            while unsafe { pause_vk[&CONVAR.config.pause_bind].is_pressed() } {
                if spfloop {
                    unsafe { PAUSED = !PAUSED };
                    spfloop = false;
                }
            }

            let mut sjfloop = true;
            while unsafe { jump_vk[&CONVAR.config.jump_bind].is_pressed() && !PAUSED} {
                if sjfloop {
                    sjfloop = false;
                    unsafe { duck_vk[&CONVAR.config.duck_bind].press(); }
                    unsafe { thread::sleep(Duration::from_millis(CONVAR.config.delay)) }
                    unsafe { duck_vk[&CONVAR.config.duck_bind].release(); }
                }
            }
        }
    });

    let options = eframe::NativeOptions {
        initial_window_size: Some(egui::vec2(320.0, 240.0)),
        ..Default::default()
    };
    eframe::run_native(
        "cjump+",
        options,
        Box::new(|_cc| Box::<MyApp>::default()),
    )
}

fn create_jump_hash() -> HashMap<String, Keyboard> {
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

fn create_duck_hash() -> HashMap<String, Keyboard> {
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

fn create_pause_hash() -> HashMap<String, Keyboard> {
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


struct MyApp {
    dark: u8,
}

impl Default for MyApp {
    fn default() -> Self {
        Self {
            dark: unsafe { CONVAR.dark_mode },
        }
    }
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            
            // Light/Dark Mode
            if self.dark != unsafe{ CONVAR.dark_mode } || unsafe{FLOOP} {
                unsafe {
                    CONVAR.dark_mode = self.dark;
                    FLOOP = false;
                };

                match self.dark {
                    1 => ctx.set_visuals(Visuals::light()),
                    0 => ctx.set_visuals(Visuals::dark()),
                    _ => todo!(),
                }
            }
            // Menu bar
            menu::bar(ui, |ui| {
                ui.menu_button("Visuals", |ui| {
                    if ui.button("Dark mode").clicked() {
                        match self.dark {
                            1 => self.dark = 0,
                            0 => self.dark = 1,
                            _ => todo!()
                        }
                    }
                });
            });

            ui.horizontal(|ui| {
                ui.heading("CJUMP+");

                if unsafe{ !PAUSED } {
                    ui.label(egui::RichText::new("Enabled").color(egui::Color32::from_rgb(0, 255, 0)));
                } else {
                    ui.label(egui::RichText::new("Disabled").color(egui::Color32::from_rgb(255, 0, 0)));
                }
            });
            ui.separator();

            egui::ComboBox::from_label("Jump bind")
                .selected_text(format!("{:?}", unsafe { &CONVAR.config.jump_bind }))
                .show_ui(ui, |ui| {
                    ui.selectable_value(unsafe{ &mut CONVAR.config.jump_bind }, String::from("Space"), "Space");
                    ui.selectable_value(unsafe{ &mut CONVAR.config.jump_bind }, String::from("V"), "V");
                    ui.selectable_value(unsafe{ &mut CONVAR.config.jump_bind }, String::from("B"), "B");
                    ui.selectable_value(unsafe{ &mut CONVAR.config.jump_bind }, String::from("N"), "N");
                }
            );

            egui::ComboBox::from_label("Duck bind")
                .selected_text(format!("{:?}", unsafe { &CONVAR.config.duck_bind }))
                .show_ui(ui, |ui| {
                    ui.selectable_value(unsafe{ &mut CONVAR.config.duck_bind }, String::from("Left Control"), "L-CTRL");
                    ui.selectable_value(unsafe{ &mut CONVAR.config.duck_bind }, String::from("C"), "C");
                }
            );

            egui::ComboBox::from_label("Pause bind")
                .selected_text(format!("{:?}", unsafe{ &CONVAR.config.pause_bind }))
                .show_ui(ui, |ui| {
                    ui.selectable_value(unsafe{ &mut CONVAR.config.pause_bind }, String::from("Enter"), "Enter");
                    ui.selectable_value(unsafe{ &mut CONVAR.config.pause_bind }, String::from("Tab"), "Tab");
                    ui.selectable_value(unsafe{ &mut CONVAR.config.pause_bind }, String::from("Backspace"), "Backspace");
                    ui.selectable_value(unsafe{ &mut CONVAR.config.pause_bind }, String::from("Left Alt"), "L-ALT");
                    ui.selectable_value(unsafe{ &mut CONVAR.config.pause_bind }, String::from("F1"), "F1");
                    ui.selectable_value(unsafe{ &mut CONVAR.config.pause_bind }, String::from("F2"), "F2");
                    ui.selectable_value(unsafe{ &mut CONVAR.config.pause_bind }, String::from("F3"), "F3");
                    ui.selectable_value(unsafe{ &mut CONVAR.config.pause_bind }, String::from("F4"), "F4");
                    ui.selectable_value(unsafe{ &mut CONVAR.config.pause_bind }, String::from("F5"), "F5");
                    ui.selectable_value(unsafe{ &mut CONVAR.config.pause_bind }, String::from("F6"), "F6");
                    ui.selectable_value(unsafe{ &mut CONVAR.config.pause_bind }, String::from("F7"), "F7");
                    ui.selectable_value(unsafe{ &mut CONVAR.config.pause_bind }, String::from("F8"), "F8");
                    ui.selectable_value(unsafe{ &mut CONVAR.config.pause_bind }, String::from("F9"), "F9");
                    ui.selectable_value(unsafe{ &mut CONVAR.config.pause_bind }, String::from("F10"), "F10");
                    ui.selectable_value(unsafe{ &mut CONVAR.config.pause_bind }, String::from("F11"), "F11");
                    ui.selectable_value(unsafe{ &mut CONVAR.config.pause_bind }, String::from("F12"), "F12");
                }
            );

            ui.separator();
            
            ui.horizontal(|ui| {
                ui.label("Delay (ms)");
                ui.add(egui::DragValue::new(unsafe { &mut CONVAR.config.delay }).speed(1));
            });

            if ui.button("Update settings").clicked() {
                let path = std::path::Path::new(CFG_PATH);
                let mut f = std::fs::File::create(path).unwrap();

                unsafe {
                    TOML_CONVAR = CONVAR.convert_to_toml();
                    let content = toml::to_string(&TOML_CONVAR).unwrap();
                    let _ = f.write_all(content.as_bytes());
                };
            }
        });
    }
}