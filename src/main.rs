mod content;
mod job;
mod math;

use std::time;

use content::blueprint::BlueprintContent;
use eframe::egui::{panel::Side, Button, RichText, SidePanel, ViewportCommand, Widget};
use job::{Space, WallNode, WallSegment};
use math::angle::Angle;
use rust_decimal::Decimal;
use rust_decimal_macros::dec;

fn main() {
    match eframe::run_native(
        "cf-closet-cad",
        eframe::NativeOptions {
            viewport: eframe::egui::ViewportBuilder {
                title: Some("Computing Fun Closet CAD".to_owned()),
                app_id: Some("Computing Fun Closet CAD".to_owned()),
                position: None,
                inner_size: None,
                min_inner_size: Some(eframe::epaint::Vec2 { x: 720.0, y: 480.0 }),
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
                content: Box::new(BlueprintContent::default()),
                space: Space::default(),
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
    space: Space,
    content: Box<dyn Content>,
    zoom: f32,
    auto_save: time::Duration,
}

impl eframe::App for MainApp {
    fn update(&mut self, ctx: &eframe::egui::Context, frame: &mut eframe::Frame) {
        let mut wall = WallSegment::default();
        wall.nodes.push(WallNode {
            angle: Angle::from_degrees(dec!(90)),
            length: dec!(100),
        });
        wall.nodes.push(WallNode {
            angle: Angle::from_degrees(Decimal::ZERO),
            length: dec!(100),
        });
        wall.nodes.push(WallNode {
            angle: Angle::from_degrees(dec!(-90)),
            length: dec!(100),
        });
        wall.nodes.push(WallNode {
            angle: Angle::from_degrees(dec!(125)),
            length: dec!(100),
        });
        self.space.walls.push(wall);

        ctx.set_pixels_per_point(self.zoom);

        SidePanel::new(Side::Left, "menu").show(ctx, |ui| {
            ui.vertical_centered_justified(|ui| {
                if Button::new(RichText::new("Blueprint")).ui(ui).clicked() {
                    //self.content = blueprint::update;
                };
                if Button::new(RichText::new("Wall Editor")).ui(ui).clicked() {
                    //self.content = wall_editor::update;
                };
                if Button::new(RichText::new("Parts List")).ui(ui).clicked() {
                    //self.content = parts_list::update;
                };
                if Button::new(RichText::new("Fullscreen")).ui(ui).clicked() {
                    ctx.send_viewport_cmd(ViewportCommand::Fullscreen(
                        !ctx.input(|i| i.viewport().fullscreen.unwrap_or(false)),
                    ))
                }
            });
        });

        self.content.update(&mut self.space, ctx, frame)
    }

    fn auto_save_interval(&self) -> time::Duration {
        self.auto_save
    }

    fn save(&mut self, _storage: &mut dyn eframe::Storage) {
        println!("Saving");
    }
}

trait Content {
    fn update(&mut self, job: &mut Space, ctx: &eframe::egui::Context, frame: &mut eframe::Frame);
}
