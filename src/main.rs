use std::time;

use eframe::{
    egui::{panel::Side, Button, CentralPanel, RichText, SidePanel, Ui, ViewportCommand, Widget},
    epaint::Vec2,
};

mod blueprint;
mod parts_list;
mod wall_editor;
mod welcome;

fn main() {
    match eframe::run_native(
        "cf-closet-cad",
        eframe::NativeOptions {
            viewport: eframe::egui::ViewportBuilder {
                title: Some("Computing Fun Closet CAD".to_owned()),
                app_id: Some("org.computingfun.cf-closet-cad".to_owned()),
                position: None,
                inner_size: None,
                min_inner_size: Some(Vec2 { x: 500.0, y: 500.0 }),
                max_inner_size: None,
                clamp_size_to_monitor_size: Some(true),
                fullscreen: None,
                maximized: Some(true),
                resizable: None,
                transparent: None,
                decorations: None,
                icon: None,
                active: None,
                visible: None,
                fullsize_content_view: None,
                title_shown: None,
                titlebar_buttons_shown: None,
                titlebar_shown: None,
                drag_and_drop: None,
                taskbar: None,
                close_button: None,
                minimize_button: None,
                maximize_button: None,
                window_level: None,
                mouse_passthrough: None,
                window_type: None,
            },
            vsync: true,
            multisampling: 0,
            depth_buffer: 0,
            stencil_buffer: 0,
            hardware_acceleration: eframe::HardwareAcceleration::Preferred,
            renderer: eframe::Renderer::Glow,
            run_and_return: true,
            event_loop_builder: None,
            window_builder: None,
            shader_version: None,
            centered: true,
            persist_window: true,
            persistence_path: None,
            dithering: true,
        },
        Box::new(|_cc| {
            Ok(Box::new(MainApp {
                content: welcome::update,
                zoom: 2.5,
                auto_save: time::Duration::from_secs(30),
            }))
        }),
    ) {
        Ok(_) => {}
        Err(err) => match err {
            eframe::Error::AppCreation(err) => eprintln!("{}", err),
            eframe::Error::Winit(err) => eprintln!("{}", err),
            eframe::Error::WinitEventLoop(err) => eprintln!("{}", err),
            eframe::Error::Glutin(err) => eprintln!("{}", err),
            eframe::Error::NoGlutinConfigs(_, err) => eprintln!("{}", err),
            eframe::Error::OpenGL(err) => eprintln!("{}", err),
        },
    }
}

struct MainApp {
    content: fn(&mut MainApp, &mut Ui) -> (),
    zoom: f32,
    auto_save: time::Duration,
}

impl eframe::App for MainApp {
    fn update(&mut self, ctx: &eframe::egui::Context, _frame: &mut eframe::Frame) {
        ctx.set_pixels_per_point(self.zoom);

        // menu
        SidePanel::new(Side::Left, "menu").show(ctx, |ui| {
            ui.vertical_centered_justified(|ui| {
                if Button::new(RichText::new("Blueprint")).ui(ui).clicked() {
                    self.content = blueprint::update;
                };
                if Button::new(RichText::new("Wall Editor")).ui(ui).clicked() {
                    self.content = wall_editor::update;
                };
                if Button::new(RichText::new("Parts List")).ui(ui).clicked() {
                    self.content = parts_list::update;
                };
                if Button::new(RichText::new("Fullscreen")).ui(ui).clicked() {
                    ctx.send_viewport_cmd(ViewportCommand::Fullscreen(
                        !ctx.input(|i| i.viewport().fullscreen.unwrap_or(false)),
                    ))
                }
            });
        });

        // content
        CentralPanel::default().show(ctx, |ui| {
            (self.content)(self, ui);
        });
    }

    fn auto_save_interval(&self) -> time::Duration {
        self.auto_save
    }

    fn save(&mut self, _storage: &mut dyn eframe::Storage) {
        println!("Saving");
    }
}
