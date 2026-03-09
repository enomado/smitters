use egui::{Color32, Stroke};
use itertools_num::linspace;

use crate::context::Context;

pub fn draw_smith_fancy(ctx: &Context) {
    let stroke = Stroke {
        width: ctx.fixed_stroke(0.5),
        color: Color32::RED,
    };

    draw_left_segment(ctx, stroke);

    ctx.draw_patch(0.5, 0.2, 4, 11);
    ctx.draw_patch(1.0, 0.5, 6, 21);
    ctx.draw_patch(2.0, 1.0, 6, 21);
    ctx.draw_patch(5.0, 2.0, 4, 11);
    ctx.draw_patch(10.0, 5.0, 3, 5);
    ctx.draw_patch(20.0, 10.0, 3, 5);
    ctx.draw_patch(50.0, 20.0, 3, 5);

    ctx.horizontal_line();
    ctx.unit_circle();
}

fn draw_left_segment(ctx: &Context<'_>, stroke: Stroke) {
    for n in 0..10i32 {
        let res = 0.02 * n as f64;
        ctx.res(res, -0.2, 0.2, &stroke);
    }
    for n in -10..10i32 {
        if n == 0 {
            continue;
        }
        let react = 0.02 * n as f64;
        ctx.react(react, 0.0, 0.2, &stroke);
    }
}

pub fn draw_patch(ctx: &Context, ending: f64, starting: f64, step: usize, step_m: usize) {
    let stroke = Stroke {
        width: ctx.fixed_stroke(0.5),
        color: Color32::RED,
    };

    for res in linspace(starting, ending, step) {
        ctx.res(res, -ending, ending, &stroke);
    }
    for react in linspace(-ending, ending, step_m) {
        if react == 0.0 {
            continue;
        }
        ctx.react(react, starting, ending, &stroke);
    }
    for res in linspace(0.0, starting, step) {
        ctx.res(res, starting, ending, &stroke);
    }
    for res in linspace(0.0, starting, step) {
        ctx.res(res, -starting, -ending, &stroke);
    }
    for react in linspace(starting, ending, step) {
        if react == 0.0 {
            continue;
        }
        ctx.react(react, 0.0, starting, &stroke);
    }
    for react in linspace(-starting, -ending, step) {
        if react == 0.0 {
            continue;
        }
        ctx.react(react, 0.0, starting, &stroke);
    }
}
