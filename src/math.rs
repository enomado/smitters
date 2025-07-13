// https://github.com/germi/vue-smith-chart/blob/master/src/components/sm-point.vue#L25
// https://www.allaboutcircuits.com/technical-articles/mathematical-construction-and-properties-of-the-smith-chart/

use std::ops::Add;

#[derive(Debug, Clone, Copy)]
pub struct ResActive(pub f64);

#[derive(Debug, Clone, Copy)]
pub struct ResReactive(pub f64);

#[derive(Debug, Clone, Copy)]
pub struct Pos2 {
    pub x: f64,
    pub y: f64,
}

impl Add for Pos2 {
    type Output = Pos2;

    fn add(self, rhs: Self) -> Self::Output {
        Pos2 {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Circle {
    pub center: Pos2,
    pub radius: f64,
}

#[derive(Debug, Clone, Copy)]
pub struct Angle(pub f64);

pub fn resistance_to_xy(res: ResActive, react: ResReactive, radius: f64) -> Pos2 {
    // Currentry we are drawing over circles and arcs, but it could be much faster to iterate over this
    let cx = {
        let r = res.0;
        let x = react.0;

        let a = (r.powi(2) - 1. + x.powi(2)) / ((r + 1.0).powi(2) + x.powi(2));

        a * radius + radius
    };

    let cy = {
        let r = res.0;
        let x = react.0;

        let b = (2. * x) / ((r + 1.).powi(2) + x.powi(2));

        -b * radius + radius
    };

    Pos2 { x: cx, y: cy }
}

pub fn res_circle(res: ResActive, radius: f64) -> Circle {
    let cx = { res.0 * radius / (res.0 + 1.) + radius };

    let cy = { radius };

    let r = { radius / (res.0 + 1.) };

    let pos = Pos2 { x: cx, y: cy };

    Circle {
        center: pos,
        radius: r,
    }
}

pub fn react_circle(react: ResReactive, radius: f64) -> Circle {
    let magnitude = { react.0.abs() };

    let cx = { radius + radius };
    let cy = { radius / react.0 + radius };

    let r = { radius / magnitude };

    let pos = Pos2 { x: cx, y: cy };

    Circle {
        center: pos,
        radius: r,
    }
}

pub fn xy_to_resistance(xy: &Pos2) -> (ResActive, ResReactive) {
    // perhaps
    // x: [-1:1]
    // y: [-1:1]

    let b = xy.y;
    let a = xy.x;

    // solve (a-1)^2 + (b-1/x)^2 = (1/x)^2 for x

    // b!=0;
    let x = (2. * b) / (a.powi(2) - 2. * a + b.powi(2) + 1.);

    let r = {
        // for some reason
        let x = x.abs();
        let b = b.abs();

        ((-b * x * (b * x - 2.)).sqrt() - b) / b
    };

    (ResActive(r), ResReactive(x))
}
