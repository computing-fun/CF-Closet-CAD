use eframe::egui::{Label, RichText, Ui, Widget};

use crate::MainApp;

pub fn update(_app: &mut MainApp, ui: &mut Ui) {
    Label::new(RichText::new("Blueprint screen")).ui(ui);
}
