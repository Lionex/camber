/*!
Plot easing functions to show examples of their capabilities
 */

extern crate plotlib;
extern crate camber;

use plotlib::style::Line;
use plotlib::page::Page;
use plotlib::view::View;
use plotlib::function::{ Function, Style };
use camber::poly_eval;
use camber::ease::*;

/// `poly_eval` can be used to create simple polynomials from their coefficients
///
/// Can calculate each value in `O(n)` multiplications, for `n` coefficients, or `n`-1 degree
/// polynomials.
fn poly_eval_demo() {
    // Plot linear, cubic, and quartic equations together
    let linear = [1., 0.];
    let quadratic = [1., 0., 0.];
    let cubic = [1., 0., 0., 0.];

    let linear = Function::new(|x| poly_eval(&linear, x), -1., 1.)
        .style(Style::new().colour("red"));
    let quadratic = Function::new(|x| poly_eval(&quadratic, x), -1., 1.)
        .style(Style::new().colour("green"));
    let cubic = Function::new(|x| poly_eval(&cubic, x), -1., 1.)
        .style(Style::new().colour("blue"));

    let v = View::new()
        .add(&linear)
        .add(&quadratic)
        .add(&cubic);
    Page::single(&v).save("img/polyeval_1.svg");

    let complicated = [-2., -1., 1., -0.1];
    let complicated = Function::new(|x| poly_eval(&complicated, x), -1., 1.)
        .style(Style::new().colour("blue"));
    let v = View::new().add(&complicated);
    Page::single(&v).save("img/polyeval_2.svg");
}

macro_rules! plot {
    ($($f:ident),+) => {
        $(
            // Define the function to plot from the given identifier
            let f = Function::new(|t| $f(t), 0., 1.)
            .style(Style::new().colour("red"));

            // Add the function to the view
            let v = View::new().add(&f);

            // Write out to a file
            Page::single(&v).save(concat!("img/", stringify!($f), ".svg"));
        )+
    }
}

fn main() {
    poly_eval_demo();

    // Plot and save the smooth start functions
    plot! {
        smooth_start_2,
        smooth_start_3,
        smooth_start_4,
        smooth_start_5,
        smooth_start_6,
        smooth_start_7,
        smooth_start_8,
        smooth_start_9
    }

    // Plot and save the smooth stop functions
    plot! {
        smooth_stop_2,
        smooth_stop_3,
        smooth_stop_4,
        smooth_stop_5,
        smooth_stop_6,
        smooth_stop_7,
        smooth_stop_8,
        smooth_stop_9
    }

    // Plot and save the smooth step functions
    plot! {
        smooth_step_2,
        smooth_step_3,
        smooth_step_4,
        smooth_step_5,
        smooth_step_6,
        smooth_step_7,
        smooth_step_8,
        smooth_step_9
    }
}
