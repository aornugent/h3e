extern crate libh3;

use libh3::{rads_to_degs, degs_to_rads};

fn main() {
    println!("Hello {}", rads_to_degs(1.002));
    println!("Hssso {   }", degs_to_rads(57.310));
}
