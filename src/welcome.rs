use eframe::egui::{Label, RichText, Ui, Widget};

use crate::MainApp;

pub fn update(_app: &mut MainApp, ui: &mut Ui) {
    Label::new(RichText::new(
        "Select a menu option on left side to get started :)",
    ))
    .ui(ui);
}
