use egui::{Color32, Stroke};
use itertools_num::linspace;

use crate::context::Context;

pub fn draw_smith_fancy(ctx: &Context) {
    let stroke = Stroke {
        width: 0.5,
        color: Color32::RED,
    };

    draw_left_segment(ctx, stroke);

    let starting = 0.2;
    let ending = 0.5;

    ctx.draw_patch(ending, starting, 4, 5 * 2 + 1);

    let starting = 0.5;
    let ending = 1.0;

    ctx.draw_patch(ending, starting, 6, 6 * 3 + 3);

    let starting = 1.0;
    let ending = 2.0;

    ctx.draw_patch(ending, starting, 6, 6 * 3 + 3);

    let starting = 2.;
    let ending = 5.0;

    ctx.draw_patch(ending, starting, 3 + 1, 5 * 2 + 1);

    ctx.horizontal_line();
}

fn draw_left_segment(ctx: &Context<'_>, stroke: Stroke) {
    for n in 0..10i32 {
        let res = (0.02 * n as f32) as f64;
        ctx.res(res, -0.2, 0.2, &stroke);
    }
    for n in -10..10i32 {
        if n == 0 {
            continue;
        }

        let react = 0.02 * n as f64;

        let res_start = 0.0;
        let res_end = 0.2;

        ctx.react(react, res_start, res_end, &stroke);
    }
}

pub fn draw_patch(ctx: &Context, ending: f64, starting: f64, step: usize, step_m: usize) {
    let stroke = Stroke {
        width: 0.5,
        color: Color32::RED,
    };

    // for n in 4..10i32 {
    for n in linspace(starting, ending, step) {
        // let res = (0.05 * n as f32) as f64;
        let res = n;
        ctx.res(res, -ending, ending, &stroke);
    }
    for n in linspace(-ending, ending, step_m) {
        if n == 0.0 {
            continue;
        }
        let react = n;
        // let react = 0.05 * n as f64;

        let res_start = starting;
        let res_end = ending;

        ctx.react(react, res_start, res_end, &stroke);
    }
    // for n in 0..4i32 {
    for n in linspace(0., starting, step) {
        let res = n;
        // let res = (0.05 * n as f32) as f64;
        ctx.res(res, starting, ending, &stroke);
    }
    // for n in 0..4i32 {
    for n in linspace(0., starting, step) {
        // let res = (0.05 * n as f32) as f64;
        let res = n;
        ctx.res(res, -starting, -ending, &stroke);
    }
    // for n in 4..10i32 {
    for n in linspace(starting, ending, step) {
        if n == 0.0 {
            continue;
        }

        // let react = 0.05 * n as f64;
        let react = n;

        let res_start = 0.0;
        let res_end = starting;

        ctx.react(react, res_start, res_end, &stroke);
    }

    // for n in -10..-4i32 {
    for n in linspace(-starting, -ending, step) {
        if n == 0.0 {
            continue;
        }

        let react = n;

        // let react = 0.05 * n as f64;

        let res_start = 0.0;
        let res_end = starting;

        ctx.react(react, res_start, res_end, &stroke);
    }
}
