use egui::{pos2, Color32, Painter, Stroke};

use crate::{
    chart_fancy::draw_patch,
    drawing_primitives::{react_circle_arc, res_circle_arc},
    egui::egui_draw_arc,
    math::{Pos2, ResActive, ResReactive},
};
pub struct Context<'a> {
    pub painter: &'a Painter,
    // not a real center
    pub center_of_image: Pos2,
    pub half_width: f64,
}

impl<'a> Context<'a> {
    pub fn get_context(ui: &'a egui::Ui) -> Context<'a> {
        let size = ui.available_rect_before_wrap();

        let center = size.center();

        let half_width = size.width() as f64 / 2.;

        let center_of_image = Pos2 {
            x: center.x as f64 - half_width,
            y: center.y as f64 - half_width,
        };

        let painter = ui.painter();

        Context {
            painter,
            half_width,
            center_of_image,
        }
    }

    pub fn center(&self) -> Pos2 {
        Pos2 {
            x: (self.center_of_image.x + self.half_width),
            y: (self.center_of_image.y + self.half_width),
        }
    }

    pub fn res(&self, res: f64, react_start: f64, react_end: f64, stroke: &Stroke) {
        let arc = res_circle_arc(
            &self.center_of_image,
            self.half_width,
            ResActive(res),
            ResReactive(react_start),
            ResReactive(react_end),
        );

        egui_draw_arc(self.painter, arc, stroke);
    }

    pub fn react(&self, react: f64, res_start: f64, res_end: f64, stroke: &Stroke) {
        let arc = react_circle_arc(
            &self.center_of_image,
            self.half_width,
            ResReactive(react),
            ResActive(res_start),
            ResActive(res_end),
        );

        egui_draw_arc(self.painter, arc, stroke)
    }

    pub fn draw_patch(&self, ending: f64, starting: f64, steps: usize, steps_m: usize) {
        draw_patch(self, ending, starting, steps, steps_m);
    }

    pub fn horizontal_line(&self) {
        let start = pos2(
            (self.center_of_image.x) as f32,
            (self.center_of_image.y + self.half_width) as f32,
        );
        let end = pos2(
            (self.center_of_image.x + self.half_width + self.half_width) as f32,
            (self.center_of_image.y + self.half_width) as f32,
        );
        self.painter.line_segment(
            [start, end],
            Stroke {
                width: 0.5,
                color: Color32::RED,
            },
        );
    }
}
