#![allow(dead_code)]

mod jobs;

use cc_core::JobNumber;
use egui::Key;

const APP_NAME: &'static str = "CC shortcuts";

const NATIVE_OPTIONS: eframe::NativeOptions = eframe::NativeOptions {
    always_on_top: false,
    maximized: false,
    decorated: true,
    fullscreen: false,
    drag_and_drop_support: true,
    icon_data: None,
    initial_window_pos: None,
    initial_window_size: None,
    min_window_size: None,
    max_window_size: None,
    resizable: true,
    transparent: true,
    mouse_passthrough: false,
    active: true,
    vsync: false,
    multisampling: 0,
    depth_buffer: 0,
    stencil_buffer: 0,
    hardware_acceleration: eframe::HardwareAcceleration::Preferred,
    renderer: eframe::Renderer::Glow,
    follow_system_theme: true,
    default_theme: eframe::Theme::Dark,
    run_and_return: true,
    event_loop_builder: None,
    shader_version: None,
    centered: false,
    app_id: None,
};

fn main() {
    let result = eframe::run_native(
        APP_NAME,
        NATIVE_OPTIONS,
        Box::new(|_context| Box::new(MainApp::default())),
    );

    if let Err(err) = result {
        eprintln!("{err}")
    }
}

#[derive(Default)]
struct MainApp {
    edit_field: String,
}

impl eframe::App for MainApp {
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            egui::widgets::TextEdit::singleline(&mut self.edit_field)
                .desired_width(f32::INFINITY)
                .show(ui);
        });
    }
}

/*
if let Some(sub_app) = &mut self.sub_app {
            return match sub_app {
                SubApps::Note(n) => n.update(ctx, frame),
            };
        }

        let typed_job = self.job_number_field.parse::<JobNumber>().ok();
        match typed_job {
            Some(num) => self.job_number = Some(num),
            None => match self.job_number_field.is_empty() {
                true => self.job_number = None,
                false => {
                    self.job_number_field = self
                        .job_number
                        .map_or_else(String::new, |num| num.to_string())
                }
            },
        }
        let copied_job = jobs::from_clipboard();

        egui::CentralPanel::default().show(ctx, |ui| {
            ui.horizontal_top(|ui| {
                ui.label("o | Open job ");
                ui.text_edit_singleline(&mut self.job_number_field);
            });

            ui.horizontal_top(|ui| {
                ui.label("v | Open job from clipboard");
                match copied_job {
                    Some(num) => ui.label(num.to_string()),
                    None => ui.label("Job number not copied"),
                };
            });

            ui.horizontal(|ui| ui.label("n | Open notes"));
        });

        ctx.input(|input| {
            if input.key_pressed(Key::V) {
                jobs::open(copied_job).ok();
            }

            if input.key_pressed(Key::O) {
                jobs::open(typed_job).ok();
            }

            if input.key_pressed(Key::N) {
                self.sub_app = Some(SubApps::Note(NoteApp {}));
            }
        });
*/
