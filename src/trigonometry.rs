use crate::math::{Angle, Circle};

/// mostly by this https://lucidar.me/en/mathematics/how-to-calculate-the-intersection-points-of-two-circles/

pub fn find_chord(c1: Circle, c2: Circle) -> f64 {
    let Circle {
        center: p1,
        radius: r1,
    } = c1;

    let Circle {
        center: p2,
        radius: r2,
    } = c2;

    let d = ((p2.x - p1.x) * (p2.x - p1.x) + (p2.y - p1.y) * (p2.y - p1.y)).sqrt();

    let a = (r1 * r1 - r2 * r2 + d * d) / (2. * d);

    let h = (r1 * r1 - a * a).sqrt();
    h
}

pub fn chorde_angle(c: f64, r: f64) -> Angle {
    let lenght = 2. * (c / r).asin();
    Angle(lenght)
}

pub fn intersect_angle_react_by_res(react: Circle, res: Circle) -> Angle {
    let h = find_chord(res, react);

    let c = h;
    let r = react.radius;

    chorde_angle(c, r)
}

// fixed
pub fn intersect_angle_res_by_react(res: Circle, react: Circle) -> Angle {
    let h = find_chord(res, react);

    let c = h;
    let r = res.radius;

    chorde_angle(c, r)
}
