use std::fmt::{Debug, Formatter};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Copy, Clone, Hash, Ord, PartialOrd, Eq, PartialEq)]
pub enum Entry {
    B(u32),
    R(u32)
}

impl Debug for Entry {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Entry::B(val) => { f.write_fmt(format_args!("B{}", val)) }
            Entry::R(val) => { f.write_fmt(format_args!("R{}", val)) }
        }
    }
}