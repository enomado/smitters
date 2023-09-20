use egui::{Color32, Stroke};

use crate::context::Context;

pub fn draw_smith_simple(ui: &mut egui::Ui) {
    let ctx = Context::get_context(ui);

    let stroke = Stroke {
        width: 0.5,
        color: Color32::RED,
    };

    for n in 0..10i32 {
        let react = 0.2 * n as f64;

        let res_start = -100.;
        let res_end = 100.;

        ctx.res(react, res_start, res_end, &stroke);
    }

    for n in -100..100i32 {
        if n == 0 {
            continue;
        }

        let react = 0.2 * n as f64;

        let res_start = 0.;
        let res_end = 1000.;

        ctx.react(react, res_start, res_end, &stroke);
    }

    ctx.horizontal_line();
}
