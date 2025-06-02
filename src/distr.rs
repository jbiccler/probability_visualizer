use std::ops::RangeInclusive;

use egui::emath::Numeric;
pub mod cont;
pub mod disc;

pub const BARELY_POSITIVE: f64 = 0.001;

#[derive(Debug, Clone)]
pub enum MixedParam {
    Float { param: Param<f64> },
    Unsigned { param: Param<u64> },
    Signed { param: Param<i64> },
}

#[derive(Clone, PartialEq, Debug)]
pub struct Param<T> {
    pub default: T,
    pub range: RangeInclusive<T>,
    pub name: String,
}
