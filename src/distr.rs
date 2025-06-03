use statrs::statistics::Distribution;
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

#[derive(Clone, PartialEq, Debug)]
pub struct SummaryStats {
    pub mean: Option<f64>,
    pub variance: Option<f64>,
    pub std_dev: Option<f64>,
    pub entropy: Option<f64>,
    pub skewness: Option<f64>,
}

impl SummaryStats {
    pub fn new(distr: &dyn Distribution<f64>) -> SummaryStats {
        SummaryStats {
            mean: distr.mean(),
            variance: distr.variance(),
            std_dev: distr.std_dev(),
            entropy: distr.entropy(),
            skewness: distr.skewness(),
        }
    }
    pub fn display_mean(&self) -> String {
        match self.mean {
            None => "N/A".to_owned(),
            Some(f) => format!("{:.3}", f),
        }
    }
    pub fn display_variance(&self) -> String {
        match self.variance {
            None => "N/A".to_owned(),
            Some(f) => format!("{:.3}", f),
        }
    }
    pub fn display_std_dev(&self) -> String {
        match self.std_dev {
            None => "N/A".to_owned(),
            Some(f) => format!("{:.3}", f),
        }
    }
    pub fn display_entropy(&self) -> String {
        match self.entropy {
            None => "N/A".to_owned(),
            Some(f) => format!("{:.3}", f),
        }
    }
    pub fn display_skewness(&self) -> String {
        match self.skewness {
            None => "N/A".to_owned(),
            Some(f) => format!("{:.3}", f),
        }
    }
}
