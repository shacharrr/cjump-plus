#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release


use std::{thread, io::Write};
use std::time::Duration;
use std::fs;

use eframe::egui;
use egui::{menu, Visuals};

mod util;
use util::{TomlConVar, ConVar};

const CFG_PATH: &str = "./cfg/cjump.toml";
// Shity way should improve
static mut FLOOP: bool = true;
static mut PAUSED: bool = false;

static mut TOML_CONVAR: TomlConVar = TomlConVar::new();
static mut CONVAR: ConVar =  ConVar::new();


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
        let jump_vk = util::create_jump_hash();
        let duck_vk = util::create_duck_hash();
        let pause_vk = util::create_pause_hash();
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