#![allow(incomplete_features, unused)]
#![feature(generic_const_exprs)]
mod defs;
mod qty;
mod ratio;
mod unit;

use defs::{Coulomb, Meter, Newton, Second, Volt};
use qty::Qty;

fn main() {
    let distance = Qty::<f64, Meter>::new(1.0);
    let time = Qty::<f64, Second>::new(2.0);

    assert_eq!(format!("{}", distance * distance), "1 [4 / 1]");

    assert_eq!(format!("{}", distance), "1 [2 / 1]");
    assert_eq!(format!("{}", (distance / time) * time), "1 [2 / 1]");

    assert_eq!(format!("{}", distance / time), "0.5 [2 / 5]");
    assert_eq!(format!("{}", distance / time / time), "0.25 [2 / 25]");

    let force = Qty::<f64, Newton>::new(1.0);
    let charge = Qty::<f64, Coulomb>::new(1.0);
    let voltage = Qty::<f64, Volt>::new(1.0);

    assert_eq!(format!("{}", force / charge), "1 [6 / 875]");
    assert_eq!(format!("{}", voltage / distance), "1 [6 / 875]");

    // Adding two quantities with different units fails to compile
    // distance + time;
}
