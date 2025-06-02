use std::ops::RangeInclusive;

pub mod cont_distr;
pub mod disc_distr;

pub const BARELY_POSITIVE: f64 = 0.001;

#[derive(Debug, Clone)]
pub enum MixedParam {
    Float { param: Param<f64> },
    Unsigned { param: Param<u64> },
    _Signed { param: Param<i64> },
}

#[derive(Clone, PartialEq, Debug)]
pub struct Param<T> {
    pub default: T,
    pub range: RangeInclusive<T>,
    pub name: String,
}
