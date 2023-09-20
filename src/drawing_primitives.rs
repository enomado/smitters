use crate::{
    math::{react_circle, res_circle, Angle, Circle, Pos2, ResActive, ResReactive},
    trigonometry::{intersect_angle_react_by_res, intersect_angle_res_by_react},
};

#[derive(Debug)]
pub struct CircleArc {
    pub circle: Circle,
    pub start: Angle,
    pub lenght: Angle,
}

fn res_circle_params(res: ResActive, center_of_image: &Pos2, half_width: f64) -> Circle {
    let mut circle = res_circle(res, half_width);
    circle.center = circle.center + *center_of_image;
    circle
}

fn react_circle_params(res: ResReactive, center_of_image: &Pos2, half_width: f64) -> Circle {
    let mut circle = react_circle(res, half_width);
    circle.center = circle.center + *center_of_image;
    circle
}

pub fn react_circle_arc(
    center_of_image: &Pos2,
    half_width: f64,
    react: ResReactive,
    res_start: ResActive,
    res_end: ResActive,
) -> CircleArc {
    // assert!(res_start.0 > 0.0);
    // assert!(res_end.0 > 0.0);

    let react_circle = react_circle_params(react, &center_of_image, half_width);

    let res_circle_1 = res_circle_params(res_start, &center_of_image, half_width);

    let start = intersect_angle_react_by_res(react_circle, res_circle_1);

    let res_circle_2 = res_circle_params(res_end, &center_of_image, half_width);

    let end = intersect_angle_react_by_res(react_circle, res_circle_2);

    let lenght = end;

    if react.0.is_sign_negative() {
        let true_start = Angle(std::f64::consts::PI / 2. + start.0);

        let lenght = Angle(lenght.0 - start.0);

        CircleArc {
            circle: react_circle,
            start: true_start,
            lenght,
        }
    } else {
        let true_start = Angle(-std::f64::consts::PI / 2. - start.0);

        let lenght = Angle(-lenght.0 + start.0);

        CircleArc {
            circle: react_circle,
            start: true_start,
            lenght,
        }
    }
}

pub fn res_circle_arc(
    center_of_image: &Pos2,
    half_width: f64,
    res: ResActive,
    react_start: ResReactive,
    react_end: ResReactive,
) -> CircleArc {
    let res_circle = res_circle_params(res, &center_of_image, half_width);

    // todo inf
    let true_start = if react_start.0 != 0.0 {
        let react_circle_1 = react_circle_params(react_start, &center_of_image, half_width);

        let start = intersect_angle_res_by_react(res_circle, react_circle_1);

        let true_start = if react_start.0.is_sign_negative() {
            start
        } else {
            Angle(std::f64::consts::TAU - start.0)
        };

        true_start
    } else {
        Angle(std::f64::consts::PI)
    };

    let react_circle_2 = react_circle_params(react_end, &center_of_image, half_width);

    let end = intersect_angle_res_by_react(res_circle, react_circle_2);

    let end = if react_end.0 != 0.0 {
        let true_end = if react_end.0.is_sign_negative() {
            Angle(end.0)
        } else {
            Angle(std::f64::consts::TAU - end.0)
        };

        true_end
    } else {
        Angle(std::f64::consts::PI)
    };

    let lenght = Angle(end.0 - true_start.0);

    CircleArc {
        circle: res_circle,
        start: true_start,
        lenght,
    }
}
