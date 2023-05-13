extern crate core;

use crate::phase1::phase1;
use crate::phase2::phase2;

mod preferences;
mod entry;
mod pairings;
mod phase1;
mod phase2;

const DATA_DIR: &str = "data";
const OUT_DIR: &str = "out";

fn main() {
    phase1();
    phase2();
}
