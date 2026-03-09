use egui::{epaint, pos2, Color32, Shape, Stroke};

use crate::{drawing_primitives::CircleArc, math::Circle};

/// Draw arcs as Bezier curves using `kurbo`.
pub fn egui_draw_arc(painter: &egui::Painter, arc: CircleArc, stroke: &Stroke) {
    let CircleArc {
        circle,
        start: start_angle,
        length,
    } = arc;

    let Circle { center, radius } = circle;

    let center = kurbo::Point::new(center.x, center.y);

    let arc = kurbo::Arc {
        center,
        radii: kurbo::Vec2::new(radius, radius),
        start_angle: start_angle.0,
        sweep_angle: length.0,
        x_rotation: 0.0,
    };

    let start = center + kurbo::Vec2::new(
        radius * start_angle.0.cos(),
        radius * start_angle.0.sin(),
    );
    let mut prev = pos2(start.x as f32, start.y as f32);

    arc.to_cubic_beziers(0.01, |p1, p2, p3| {
        if p1.is_nan() {
            return;
        }

        let p1 = pos2(p1.x as f32, p1.y as f32);
        let p2 = pos2(p2.x as f32, p2.y as f32);
        let p3 = pos2(p3.x as f32, p3.y as f32);

        painter.add(Shape::CubicBezier(epaint::CubicBezierShape {
            points: [prev, p1, p2, p3],
            closed: false,
            fill: Color32::TRANSPARENT,
            stroke: (*stroke).into(),
        }));

        prev = p3;
    });
}
