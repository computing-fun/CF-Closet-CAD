use eframe::{
    egui::{CentralPanel, Context, Key, Painter},
    epaint::{Color32, PathStroke, Pos2, Shape::LineSegment, Vec2},
};
use rust_decimal::prelude::ToPrimitive;

use crate::{job::Space, Content};

#[derive(Default, Debug, Clone, Copy)]
pub struct BlueprintContent {
    canvas: Pos2,
    zoom: f32,
}

impl Content for BlueprintContent {
    fn update(&mut self, space: &mut Space, ctx: &Context, frame: &mut eframe::Frame) {
        ctx.input(|reader| {
            self.zoom += reader.smooth_scroll_delta.y / 50.0;
            self.zoom = self.zoom.clamp(1.0, 500.0);

            if reader.pointer.secondary_down() {
                self.canvas += reader.pointer.delta()
            }

            if reader.key_pressed(Key::Escape) {
                self.zoom = 5.0;
                self.canvas = Pos2::ZERO;
            }
        });

        CentralPanel::default().show(ctx, |ui| {
            let offset = ui.min_rect().min.to_vec2() + self.canvas.to_vec2();
            let painter = ui.painter();
            Self::draw_space(painter, &space, offset, self.zoom);
        });
    }
}

impl BlueprintContent {
    fn draw_space(painter: &Painter, space: &Space, offset: Vec2, scale: f32) {
        for segment in space.points() {
            for target in segment.windows(2) {
                painter.add(LineSegment {
                    points: [
                        (Pos2::new(
                            target[0].0.to_f32().unwrap_or_default(),
                            target[0].1.to_f32().unwrap_or_default(),
                        ) + offset)
                            * scale,
                        (Pos2::new(
                            target[1].0.to_f32().unwrap_or_default(),
                            target[1].1.to_f32().unwrap_or_default(),
                        ) + offset)
                            * scale,
                    ],
                    stroke: PathStroke::new(4.5, Color32::WHITE),
                });
            }
        }
    }
}
