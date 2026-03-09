#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use eframe::egui;
use egui::{pos2, Color32, RichText, Stroke};
use smitters::{
    chart_fancy::draw_smith_fancy,
    chart_simple::draw_smith_simple_ctx,
    context::Context,
    math::{resistance_to_xy, xy_to_resistance, Pos2, ResActive, ResReactive},
};

fn main() -> Result<(), eframe::Error> {
    env_logger::init();
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([800.0, 700.0])
            .with_title("smitters — Smith Chart Demo"),
        ..Default::default()
    };
    eframe::run_native(
        "smitters — Smith Chart Demo",
        options,
        Box::new(|_cc| Ok(Box::new(DemoApp::default()))),
    )
}

struct DemoApp {
    fancy: bool,
    show_crosshair: bool,
    resistance: f64,
    reactance: f64,
    show_point: bool,
}

impl Default for DemoApp {
    fn default() -> Self {
        Self {
            fancy: true,
            show_crosshair: true,
            resistance: 1.0,
            reactance: 1.0,
            show_point: true,
        }
    }
}

impl eframe::App for DemoApp {
    fn update(&mut self, egui_ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::SidePanel::left("controls").min_width(200.0).show(egui_ctx, |ui| {
            ui.heading("smitters");
            ui.label(RichText::new("Smith Chart Library").italics());
            ui.separator();

            ui.label("Grid style:");
            ui.radio_value(&mut self.fancy, true, "Fancy (detailed grid)");
            ui.radio_value(&mut self.fancy, false, "Simple");
            ui.separator();

            ui.checkbox(&mut self.show_crosshair, "Show hover crosshair");
            ui.separator();

            ui.checkbox(&mut self.show_point, "Show impedance point");
            if self.show_point {
                ui.horizontal(|ui| {
                    ui.label("R:");
                    ui.add(egui::DragValue::new(&mut self.resistance).speed(0.01).range(0.0..=50.0));
                });
                ui.horizontal(|ui| {
                    ui.label("X:");
                    ui.add(egui::DragValue::new(&mut self.reactance).speed(0.01).range(-50.0..=50.0));
                });
                ui.label(RichText::new(format!("Z = {:.2} + j{:.2}", self.resistance, self.reactance)).monospace());
            }
        });

        egui::CentralPanel::default().show(egui_ctx, |ui| {
            let ctx = Context::get_context(ui);

            if self.fancy {
                draw_smith_fancy(&ctx);
            } else {
                draw_smith_simple_ctx(&ctx);
            }

            // Draw hover crosshair with impedance readout
            if self.show_crosshair {
                if let Some(point) = ui.input(|i| i.pointer.hover_pos()) {
                    let p = Pos2 {
                        x: -((ctx.center().x - point.x as f64) / ctx.half_width),
                        y: -(ctx.center().y - point.y as f64) / ctx.half_width,
                    };

                    // Only show when inside the unit circle
                    if p.x * p.x + p.y * p.y <= 1.0 {
                        let (res, react) = xy_to_resistance(&p);

                        let stroke = Stroke {
                            width: ctx.fixed_stroke(1.0),
                            color: Color32::from_rgba_unmultiplied(0, 180, 0, 160),
                        };

                        ctx.react(react.0, 0.0, 1000.0, &stroke);
                        ctx.res(res.0, -1000.0, 1000.0, &stroke);

                        let painter = ui.painter();
                        painter.text(
                            pos2(point.x + 15.0, point.y - 20.0),
                            egui::Align2::LEFT_BOTTOM,
                            format!("R = {:.3}\nX = {:.3}", res.0, react.0),
                            egui::FontId::monospace(12.0),
                            Color32::WHITE,
                        );
                    }
                }
            }

            // Draw impedance point
            if self.show_point {
                let point_xy = resistance_to_xy(
                    ResActive(self.resistance),
                    ResReactive(self.reactance),
                    ctx.half_width,
                );
                let screen_pos = pos2(
                    (point_xy.x + ctx.center_of_image.x) as f32,
                    (point_xy.y + ctx.center_of_image.y) as f32,
                );

                let painter = ui.painter();
                let dot_r = ctx.fixed_stroke(5.0);
                painter.circle_filled(screen_pos, dot_r, Color32::from_rgb(50, 120, 255));
                painter.circle_stroke(screen_pos, dot_r, Stroke::new(ctx.fixed_stroke(1.5), Color32::WHITE));

                // Draw constant-R and constant-X circles through the point
                let highlight = Stroke {
                    width: ctx.fixed_stroke(1.5),
                    color: Color32::from_rgba_unmultiplied(50, 120, 255, 120),
                };
                ctx.res(self.resistance, -1000.0, 1000.0, &highlight);
                ctx.react(-self.reactance, 0.0, 1000.0, &highlight);
            }
        });
    }
}
