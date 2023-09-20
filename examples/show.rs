#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

use eframe::egui;

use egui::{Color32, Stroke};
use smitters::{
    chart_fancy::draw_smith_fancy,
    context::Context,
    math::{xy_to_resistance, Pos2},
};

fn main() -> Result<(), eframe::Error> {
    env_logger::init(); // Log to stderr (if you run with `RUST_LOG=debug`).
    let options = eframe::NativeOptions {
        initial_window_size: Some(egui::vec2(320.0, 240.0)),
        ..Default::default()
    };
    eframe::run_native(
        "Smith graph XY -> RX",
        options,
        Box::new(|_cc| Box::<MyApp>::default()),
    )
}

struct MyApp {}

impl Default for MyApp {
    fn default() -> Self {
        Self {}
    }
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            let ctx = Context::get_context(ui);

            draw_smith_fancy(&ctx);

            let p = ui.input(|x| x.pointer.hover_pos());

            if let Some(point) = p {
                // idk

                let p = Pos2 {
                    x: -((ctx.center().x - point.x as f64) / ctx.half_width),
                    y: -(ctx.center().y - point.y as f64) / ctx.half_width,
                };

                let r = xy_to_resistance(&p);

                let react = r.1;
                let res = r.0;

                let stroke = Stroke {
                    width: 1.,
                    color: Color32::GREEN,
                };

                ctx.react(react.0, 0.0, 1000., &stroke);

                ctx.res(res.0, -1000.0, 1000., &stroke);

                ui.label(format!("px {:?}", &p.x));
                ui.label(format!("py {:?}", &p.y));
                ui.label(format!("r {:?}", &r.0));
                ui.label(format!("x {:?}", &r.1));
            }
        });
    }
}
