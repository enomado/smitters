#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

use eframe::egui;
use egui::DragValue;
use smitters::{
    chart_fancy::draw_smith_fancy,
    chart_simple::draw_smith_simple,
    context::Context,
    math::{ResActive, ResReactive},
};

fn main() -> Result<(), eframe::Error> {
    env_logger::init(); // Log to stderr (if you run with `RUST_LOG=debug`).
    let options = eframe::NativeOptions {
        // initial_window_size: Some(egui::vec2(320.0, 240.0)),
        ..Default::default()
    };
    eframe::run_native(
        "Smith Diagram Sample App",
        options,
        Box::new(|_cc| Ok(Box::<MyApp>::default())),
    )
}

#[allow(dead_code)]
struct MyApp {
    active: ResActive,
    reactive: ResReactive,

    tmp1: usize,
    tmp2: usize,
}

impl Default for MyApp {
    fn default() -> Self {
        Self {
            active: ResActive(1.0),
            reactive: ResReactive(1.0),
            tmp1: 1,
            tmp2: 1,
        }
    }
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            let p = DragValue::new(&mut self.tmp1).speed(0.1);

            ui.add(p);

            let p2 = DragValue::new(&mut self.tmp2).speed(0.1);

            ui.add(p2);

            if false {
                let ctx = Context::get_context(ui);
                draw_smith_fancy(&ctx);
            } else {
                draw_smith_simple(ui);
            }

            // ui.label(format!(
            //     "{}': {}",
            //     self.reactive.0, self.active.0
            // ));
        });
    }
}
