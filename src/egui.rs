use egui::{epaint, pos2, Color32, Shape, Stroke};

use crate::{drawing_primitives::CircleArc, math::Circle};

/// Draw arcs as Bezier curves using `kurbo`
pub fn egui_draw_arc(painter: &egui::Painter, arc: CircleArc, stroke: &Stroke) {
    let CircleArc {
        circle,
        start: start_f,
        lenght,
    } = arc;

    let Circle { center, radius } = circle;

    let center = kurbo::Point {
        x: center.x,
        y: center.y,
    };

    let start = center
        + kurbo::Vec2 {
            x: radius * start_f.0.cos(),
            y: radius * start_f.0.sin(),
        };

    let p = kurbo::Arc {
        center,
        radii: kurbo::Vec2 {
            x: radius,
            y: radius,
        },
        start_angle: start_f.0,
        sweep_angle: lenght.0,
        x_rotation: 0.0,
    };

    let mut p_start = pos2(start.x as f32, start.y as f32);

    p.to_cubic_beziers(0.01, |x, y, z| {
        if x.is_nan() {
            return;
            // panic!("nan");
        }

        let p1 = pos2(x.x as f32, x.y as f32);

        let p2 = pos2(y.x as f32, y.y as f32);
        let p3 = pos2(z.x as f32, z.y as f32);

        painter.add(Shape::CubicBezier(epaint::CubicBezierShape {
            points: [p_start, p1, p2, p3],
            closed: false,
            fill: Color32::TRANSPARENT,

            stroke: (*stroke).into(),
        }));

        p_start = p3;
    });
}
