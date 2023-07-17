#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release


use mki::Keyboard;
use std::thread;
use std::time::Duration;

use ini::Ini;
use eframe::egui;
use egui::{menu, Visuals};
use std::collections::HashMap;


const CFG_PATH: &str = "./cfg/cjump.ini";
static mut DARK: bool = true;
// Shity way should improve
static mut FLOOP: bool = true;

static mut JUMP_BIND: Keyboard = Keyboard::Space;
static mut PAUSE_BIND: Keyboard = Keyboard::Enter;
static mut PAUSED: bool = false;
static mut DELAY: u64 = 850;

fn main() -> Result<(), eframe::Error> {
    match std::fs::metadata(CFG_PATH) {
        Ok(_) => {
            let mut cfg = Ini::load_from_file(CFG_PATH).unwrap();
            let mut s = cfg.with_section(Some("style"));
            let d = s.get("dark").unwrap();
            if d == "0"{
                unsafe { DARK = false };
            }

            let jump_vk = create_jump_hash();
            let mut s = cfg.with_section(Some("binds"));
            let d = s.get("jump").unwrap();
            unsafe {
                JUMP_BIND = jump_vk[d];
            };

            let pause_vk = create_pause_hash();
            let mut s = cfg.with_section(Some("binds"));
            let d = s.get("pause").unwrap();
            unsafe {
                PAUSE_BIND = pause_vk[d];
            };

            let mut s = cfg.with_section(Some("binds"));
            let d = s.get("delay").unwrap();
            unsafe {
                DELAY = d.parse().unwrap();
            };
        },
        Err(_) => {
            let path = std::path::Path::new(CFG_PATH);
            let prefix = path.parent().unwrap();
            std::fs::create_dir_all(prefix).unwrap();
            std::fs::File::create(path).unwrap();

            // Ini init
            let mut cfg = Ini::new();
            cfg.with_section(Some("style"))
                .set("dark", "1");
            cfg.with_section(Some("binds"))
                .set("jump", "Space");
            cfg.with_section(Some("binds"))
                .set("pause", "Enter");
            cfg.with_section(Some("binds"))
                .set("delay", "850");

            cfg.write_to_file(CFG_PATH).unwrap();

            unsafe {
                DARK = true;
            };
        },
    }

    thread::spawn(|| {
        loop {
            let mut spfloop = true;
            while unsafe { PAUSE_BIND.is_pressed() } {
                if spfloop {
                    unsafe { PAUSED = !PAUSED };
                    spfloop = false;
                }
            }

            let mut sjfloop = true;
            while unsafe { JUMP_BIND.is_pressed() && !PAUSED} {
                if sjfloop {
                    sjfloop = false;
                    Keyboard::LeftControl.press();
                    unsafe { thread::sleep(Duration::from_millis(DELAY)) }
                    Keyboard::LeftControl.release();
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

fn find_key_for_value<'a>(map: &'a HashMap<String, Keyboard>, value: Keyboard) -> Option<&'a String> {
    map.iter()
        .find_map(|(key, &val)| if val == value { Some(key) } else { None })
}

fn create_jump_hash() -> HashMap<String, Keyboard> {
    let mut jump_vk = HashMap::new();
    jump_vk.insert(
        "Space".to_string(),
        Keyboard::Space,
    );
    jump_vk.insert(
        "C".to_string(),
        Keyboard::C,
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
    dark: bool,
    jump_vk: HashMap<String, Keyboard>,
    jump_select: String,
    pause_vk: HashMap<String, Keyboard>,
    pause_select: String,
    delay: u64
}


impl Default for MyApp {
    fn default() -> Self {
        let jump_vk = create_jump_hash();
        let pause_vk = create_pause_hash();
        let jc = jump_vk.clone();
        let cp = pause_vk.clone();
        Self {
            dark: unsafe { DARK },
            jump_vk: jump_vk,
            jump_select: find_key_for_value(&jc, unsafe { JUMP_BIND }).unwrap().to_string(),
            pause_vk: pause_vk,
            pause_select: find_key_for_value(&cp, unsafe { PAUSE_BIND }).unwrap().to_string(),
            delay: unsafe { DELAY },
        }
    }
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            
            // Light/Dark Mode
            if self.dark != unsafe{DARK} || unsafe{FLOOP} {
                let mut cfg = Ini::load_from_file(CFG_PATH).unwrap();
                cfg.with_section(Some("style"))
                    .set("dark", (self.dark as u8).to_string().as_str());
                cfg.write_to_file(CFG_PATH).unwrap();

                unsafe {
                    DARK = self.dark;
                    FLOOP = false;
                };

                match self.dark {
                    false => ctx.set_visuals(Visuals::light()),
                    true => ctx.set_visuals(Visuals::dark()),
                }
            }
            // Menu bar
            menu::bar(ui, |ui| {
                ui.menu_button("Visuals", |ui| {
                    ui.checkbox(&mut self.dark, "Dark mode");
                })
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
                .selected_text(format!("{:?}", self.jump_select))
                .show_ui(ui, |ui| {
                    ui.selectable_value(&mut self.jump_select, String::from("Space"), "Space");
                    ui.selectable_value(&mut self.jump_select, String::from("C"), "C");
                    ui.selectable_value(&mut self.jump_select, String::from("V"), "V");
                    ui.selectable_value(&mut self.jump_select, String::from("B"), "B");
                    ui.selectable_value(&mut self.jump_select, String::from("N"), "N");
                }
            );

            egui::ComboBox::from_label("Pause bind")
                .selected_text(format!("{:?}", self.pause_select))
                .show_ui(ui, |ui| {
                    ui.selectable_value(&mut self.pause_select, String::from("Enter"), "Enter");
                    ui.selectable_value(&mut self.pause_select, String::from("Tab"), "Tab");
                    ui.selectable_value(&mut self.pause_select, String::from("Backspace"), "Backspace");
                    ui.selectable_value(&mut self.pause_select, String::from("Left Alt"), "Left Alt");
                    ui.selectable_value(&mut self.pause_select, String::from("F1"), "F1");
                    ui.selectable_value(&mut self.pause_select, String::from("F2"), "F2");
                    ui.selectable_value(&mut self.pause_select, String::from("F3"), "F3");
                    ui.selectable_value(&mut self.pause_select, String::from("F4"), "F4");
                    ui.selectable_value(&mut self.pause_select, String::from("F5"), "F5");
                    ui.selectable_value(&mut self.pause_select, String::from("F6"), "F6");
                    ui.selectable_value(&mut self.pause_select, String::from("F7"), "F7");
                    ui.selectable_value(&mut self.pause_select, String::from("F8"), "F8");
                    ui.selectable_value(&mut self.pause_select, String::from("F9"), "F9");
                    ui.selectable_value(&mut self.pause_select, String::from("F10"), "F10");
                    ui.selectable_value(&mut self.pause_select, String::from("F11"), "F11");
                    ui.selectable_value(&mut self.pause_select, String::from("F12"), "F12");
                }
            );

            ui.separator();
            
            ui.horizontal(|ui| {
                ui.label("Delay (ms)");
                ui.add(egui::DragValue::new(&mut self.delay).speed(1));
            });

            if ui.button("Update settings").clicked() {
                let mut cfg = Ini::load_from_file(CFG_PATH).unwrap();
                cfg.with_section(Some("binds"))
                    .set("jump", self.jump_select.as_str());
                cfg.with_section(Some("binds"))
                    .set("pause", self.pause_select.as_str());
                cfg.with_section(Some("binds"))
                    .set("delay", self.delay.to_string());

                cfg.write_to_file(CFG_PATH).unwrap();

                unsafe {
                    JUMP_BIND = self.jump_vk[self.jump_select.as_str()];
                    PAUSE_BIND = self.pause_vk[self.pause_select.as_str()];
                    DELAY = self.delay;
                };
            }
        });
    }
}