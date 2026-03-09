use egui::{Color32, Stroke};

use crate::context::Context;

pub fn draw_smith_simple(ui: &egui::Ui) {
    let ctx = Context::get_context(ui);
    draw_smith_simple_ctx(&ctx);
}

pub fn draw_smith_simple_ctx(ctx: &Context) {
    let stroke = Stroke {
        width: 0.5,
        color: Color32::RED,
    };

    for n in 0..10i32 {
        let res = 0.2 * n as f64;
        ctx.res(res, -100.0, 100.0, &stroke);
    }

    for n in -100..100i32 {
        if n == 0 {
            continue;
        }
        let react = 0.2 * n as f64;
        ctx.react(react, 0.0, 1000.0, &stroke);
    }

    ctx.horizontal_line();
}
