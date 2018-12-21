extern crate plotlib;
extern crate camber;

use plotlib::style::Line;
use plotlib::page::Page;
use plotlib::view::View;
use plotlib::function::{ Function, Style };
use camber::poly_eval;

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
    Page::single(&v).save("polyeval_1.svg");

    let complicated = [-2., -1., 1., -0.1];
    let complicated = Function::new(|x| poly_eval(&complicated, x), -1., 1.)
    .style(Style::new().colour("blue"));
    let v = View::new().add(&complicated);
    Page::single(&v).save("polyeval_2.svg");
}

fn main() {
    poly_eval_demo();

}
