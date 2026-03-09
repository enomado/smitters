#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use eframe::egui;
use smitters::{
    chart_fancy::draw_smith_fancy, chart_simple::draw_smith_simple, context::Context,
};

fn main() -> Result<(), eframe::Error> {
    env_logger::init();
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([600.0, 600.0]),
        ..Default::default()
    };
    eframe::run_native(
        "smitters — Smith Chart",
        options,
        Box::new(|_cc| Ok(Box::<MyApp>::default())),
    )
}

struct MyApp {
    fancy: bool,
}

impl Default for MyApp {
    fn default() -> Self {
        Self { fancy: true }
    }
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.checkbox(&mut self.fancy, "Fancy grid");

            if self.fancy {
                let ctx = Context::get_context(ui);
                draw_smith_fancy(&ctx);
            } else {
                draw_smith_simple(ui);
            }
        });
    }
}
