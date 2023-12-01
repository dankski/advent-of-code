mod crate::util;

use util::calibration_value;



fn main() {
    let value = calibration_value();

    println!("{value}");
}
