use super::{BARELY_POSITIVE, MixedParam, Param};
use statrs::distribution::*;
use std::{fmt::Display, ops::RangeInclusive};
use strum_macros::EnumIter;

#[derive(Debug, PartialEq, PartialOrd, EnumIter, Clone)]
pub enum DistrTypes {
    Poisson,
    Binomial,
    Bernoulli,
    Geometric,
    Hypergeometric,
    NegativeBinomial,
}

impl DistrTypes {
    pub fn get_defaults(&self) -> Vec<MixedParam> {
        match self {
            DistrTypes::Poisson => vec![MixedParam::Float {
                param: Param {
                    default: 1_f64,
                    range: RangeInclusive::new(BARELY_POSITIVE, f64::MAX),
                    name: "Lambda".to_owned(),
                },
            }],
            DistrTypes::Binomial => vec![
                MixedParam::Float {
                    param: Param {
                        default: 0.5_f64,
                        range: RangeInclusive::new(0_f64, 1.0),
                        name: "p".to_owned(),
                    },
                },
                MixedParam::Unsigned {
                    param: Param {
                        default: 5_u64,
                        range: RangeInclusive::new(0, u64::MAX),
                        name: "n".to_owned(),
                    },
                },
            ],
            DistrTypes::Bernoulli => vec![MixedParam::Float {
                param: Param {
                    default: 0.5_f64,
                    range: RangeInclusive::new(0., 1.),
                    name: "p".to_owned(),
                },
            }],
            DistrTypes::Geometric => vec![MixedParam::Float {
                param: Param {
                    default: 0.5_f64,
                    range: RangeInclusive::new(BARELY_POSITIVE, 1.),
                    name: "p".to_owned(),
                },
            }],
            DistrTypes::Hypergeometric => vec![
                MixedParam::Unsigned {
                    param: Param {
                        default: 500,
                        range: RangeInclusive::new(1, u64::MAX),
                        name: "Population".to_owned(),
                    },
                },
                MixedParam::Unsigned {
                    param: Param {
                        default: 50,
                        range: RangeInclusive::new(0, u64::MAX),
                        name: "Successes".to_owned(),
                    },
                },
                MixedParam::Unsigned {
                    param: Param {
                        default: 100,
                        range: RangeInclusive::new(0, u64::MAX),
                        name: "Draws".to_owned(),
                    },
                },
            ],
            DistrTypes::NegativeBinomial => vec![
                MixedParam::Float {
                    param: Param {
                        default: 4.0,
                        range: RangeInclusive::new(0_f64, f64::MAX),
                        name: "r".to_owned(),
                    },
                },
                MixedParam::Float {
                    param: Param {
                        default: 0.5_f64,
                        range: RangeInclusive::new(0_f64, 1.0),
                        name: "p".to_owned(),
                    },
                },
            ],
        }
    }
}
impl Display for DistrTypes {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            DistrTypes::Poisson => write!(f, "Poisson"),
            DistrTypes::Binomial => write!(f, "Binomial"),
            DistrTypes::Bernoulli => write!(f, "Bernoulli"),
            DistrTypes::Geometric => write!(f, "Geometric"),
            DistrTypes::Hypergeometric => write!(f, "Hypergeometric"),
            DistrTypes::NegativeBinomial => write!(f, "NegativeBinomial"),
        }
    }
}

pub trait Disc: Discrete<u64, f64> + DiscreteCDF<u64, f64> {}
impl Disc for Binomial {}
impl Disc for Poisson {}
impl Disc for Bernoulli {}
impl Disc for Geometric {}
impl Disc for Hypergeometric {}
impl Disc for NegativeBinomial {}
