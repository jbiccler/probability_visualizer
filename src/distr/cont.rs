use super::{BARELY_POSITIVE, MixedParam, Param};
use statrs::distribution::*;
use std::{fmt::Display, ops::RangeInclusive};
use strum_macros::EnumIter;

#[derive(Debug, PartialEq, PartialOrd, EnumIter, Clone)]
pub enum DistrTypes {
    Normal,
    Gamma,
    Beta,
    Cauchy,
    ChiSquared,
    Exp,
    FisherSnedecor,
    Gumbel,
    InverseGamma,
    Laplace,
    LogNormal,
    Pareto,
    StudentsT,
    Triangular,
    Uniform,
    Weibull,
}

impl Display for DistrTypes {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            DistrTypes::Normal => write!(f, "Normal"),
            DistrTypes::Gamma => write!(f, "Gamma"),
            DistrTypes::Beta => write!(f, "Beta"),
            DistrTypes::Cauchy => write!(f, "Cauchy"),
            DistrTypes::ChiSquared => write!(f, "ChiSquared"),
            DistrTypes::Exp => write!(f, "Exp"),
            DistrTypes::FisherSnedecor => write!(f, "FisherSnedecor"),
            DistrTypes::Gumbel => write!(f, "Gumbel"),
            DistrTypes::InverseGamma => write!(f, "InverseGamma"),
            DistrTypes::Laplace => write!(f, "Laplace"),
            DistrTypes::LogNormal => write!(f, "LogNormal"),
            DistrTypes::Pareto => write!(f, "Pareto"),
            DistrTypes::StudentsT => write!(f, "StudentsT"),
            DistrTypes::Triangular => write!(f, "Triangular"),
            DistrTypes::Uniform => write!(f, "Uniform"),
            DistrTypes::Weibull => write!(f, "Weibull"),
        }
    }
}

impl DistrTypes {
    pub fn get_defaults(&self) -> Vec<MixedParam> {
        match self {
            DistrTypes::Normal => vec![
                MixedParam::Float {
                    param: Param {
                        default: 0_f64,
                        range: RangeInclusive::new(f64::MIN, f64::MAX),
                        name: "Mean".to_owned(),
                    },
                },
                MixedParam::Float {
                    param: Param {
                        default: 1_f64,
                        range: RangeInclusive::new(BARELY_POSITIVE, f64::MAX),
                        name: "Std. dev.".to_owned(),
                    },
                },
            ],
            DistrTypes::Gamma => vec![
                MixedParam::Float {
                    param: Param {
                        default: 1_f64,
                        range: RangeInclusive::new(BARELY_POSITIVE, f64::MAX),
                        name: "Shape".to_owned(),
                    },
                },
                MixedParam::Float {
                    param: Param {
                        default: 1_f64,
                        range: RangeInclusive::new(BARELY_POSITIVE, f64::MAX),
                        name: "Rate".to_owned(),
                    },
                },
            ],
            DistrTypes::Beta => vec![
                MixedParam::Float {
                    param: Param {
                        default: 2_f64,
                        range: RangeInclusive::new(BARELY_POSITIVE, f64::MAX),
                        name: "Shape A".to_owned(),
                    },
                },
                MixedParam::Float {
                    param: Param {
                        default: 2_f64,
                        range: RangeInclusive::new(BARELY_POSITIVE, f64::MAX),
                        name: "Shape B".to_owned(),
                    },
                },
            ],
            DistrTypes::Cauchy => vec![
                MixedParam::Float {
                    param: Param {
                        default: 0_f64,
                        range: RangeInclusive::new(f64::MIN, f64::MAX),
                        name: "Location".to_owned(),
                    },
                },
                MixedParam::Float {
                    param: Param {
                        default: 1_f64,
                        range: RangeInclusive::new(BARELY_POSITIVE, f64::MAX),
                        name: "Scale".to_owned(),
                    },
                },
            ],
            DistrTypes::ChiSquared => vec![MixedParam::Float {
                param: Param {
                    default: 3_f64,
                    range: RangeInclusive::new(BARELY_POSITIVE, f64::MAX),
                    name: "Freedom".to_owned(),
                },
            }],
            DistrTypes::Exp => vec![MixedParam::Float {
                param: Param {
                    default: 1_f64,
                    range: RangeInclusive::new(BARELY_POSITIVE, f64::MAX),
                    name: "Rate".to_owned(),
                },
            }],
            DistrTypes::FisherSnedecor => vec![
                MixedParam::Float {
                    param: Param {
                        default: 3_f64,
                        range: RangeInclusive::new(BARELY_POSITIVE, f64::MAX),
                        name: "Freedom 1".to_owned(),
                    },
                },
                MixedParam::Float {
                    param: Param {
                        default: 3_f64,
                        range: RangeInclusive::new(BARELY_POSITIVE, f64::MAX),
                        name: "Freedom 2".to_owned(),
                    },
                },
            ],
            DistrTypes::Gumbel => vec![
                MixedParam::Float {
                    param: Param {
                        default: 0_f64,
                        range: RangeInclusive::new(f64::MIN, f64::MAX),
                        name: "Location".to_owned(),
                    },
                },
                MixedParam::Float {
                    param: Param {
                        default: 1_f64,
                        range: RangeInclusive::new(BARELY_POSITIVE, f64::MAX),
                        name: "Scale".to_owned(),
                    },
                },
            ],
            DistrTypes::InverseGamma => vec![
                MixedParam::Float {
                    param: Param {
                        default: 1_f64,
                        range: RangeInclusive::new(BARELY_POSITIVE, f64::MAX),
                        name: "Shape".to_owned(),
                    },
                },
                MixedParam::Float {
                    param: Param {
                        default: 1_f64,
                        range: RangeInclusive::new(BARELY_POSITIVE, f64::MAX),
                        name: "Rate".to_owned(),
                    },
                },
            ],
            DistrTypes::Laplace => vec![
                MixedParam::Float {
                    param: Param {
                        default: 0_f64,
                        range: RangeInclusive::new(f64::MIN, f64::MAX),
                        name: "Location".to_owned(),
                    },
                },
                MixedParam::Float {
                    param: Param {
                        default: 1_f64,
                        range: RangeInclusive::new(BARELY_POSITIVE, f64::MAX),
                        name: "Scale".to_owned(),
                    },
                },
            ],
            DistrTypes::LogNormal => vec![
                MixedParam::Float {
                    param: Param {
                        default: 0_f64,
                        range: RangeInclusive::new(f64::MIN, f64::MAX),
                        name: "Location".to_owned(),
                    },
                },
                MixedParam::Float {
                    param: Param {
                        default: 1_f64,
                        range: RangeInclusive::new(BARELY_POSITIVE, 1000_f64),
                        name: "Scale".to_owned(),
                    },
                },
            ],
            DistrTypes::Pareto => vec![
                MixedParam::Float {
                    param: Param {
                        default: 1_f64,
                        range: RangeInclusive::new(BARELY_POSITIVE, f64::MAX),
                        name: "Scale".to_owned(),
                    },
                },
                MixedParam::Float {
                    param: Param {
                        default: 2_f64,
                        range: RangeInclusive::new(BARELY_POSITIVE, 1000_f64),
                        name: "Shape".to_owned(),
                    },
                },
            ],
            DistrTypes::StudentsT => vec![
                MixedParam::Float {
                    param: Param {
                        default: 0_f64,
                        range: RangeInclusive::new(f64::MIN, f64::MAX),
                        name: "Location".to_owned(),
                    },
                },
                MixedParam::Float {
                    param: Param {
                        default: 1_f64,
                        range: RangeInclusive::new(BARELY_POSITIVE, f64::MAX),
                        name: "Scale".to_owned(),
                    },
                },
                MixedParam::Float {
                    param: Param {
                        default: 2_f64,
                        range: RangeInclusive::new(BARELY_POSITIVE, f64::MAX),
                        name: "Freedom".to_owned(),
                    },
                },
            ],
            DistrTypes::Triangular => vec![
                MixedParam::Float {
                    param: Param {
                        default: 0_f64,
                        range: RangeInclusive::new(f64::MIN, f64::MAX),
                        name: "Min".to_owned(),
                    },
                },
                MixedParam::Float {
                    param: Param {
                        default: 5_f64,
                        range: RangeInclusive::new(f64::MIN, f64::MAX),
                        name: "Max".to_owned(),
                    },
                },
                MixedParam::Float {
                    param: Param {
                        default: 2.5,
                        range: RangeInclusive::new(f64::MIN, f64::MAX),
                        name: "Mode".to_owned(),
                    },
                },
            ],
            DistrTypes::Uniform => vec![
                MixedParam::Float {
                    param: Param {
                        default: 0_f64,
                        range: RangeInclusive::new(f64::MIN, f64::MAX),
                        name: "Min".to_owned(),
                    },
                },
                MixedParam::Float {
                    param: Param {
                        default: 1_f64,
                        range: RangeInclusive::new(f64::MIN, f64::MAX),
                        name: "Max".to_owned(),
                    },
                },
            ],
            DistrTypes::Weibull => vec![
                MixedParam::Float {
                    param: Param {
                        default: 10_f64,
                        range: RangeInclusive::new(BARELY_POSITIVE, f64::MAX),
                        name: "Shape".to_owned(),
                    },
                },
                MixedParam::Float {
                    param: Param {
                        default: 1_f64,
                        range: RangeInclusive::new(BARELY_POSITIVE, f64::MAX),
                        name: "Scale".to_owned(),
                    },
                },
            ],
        }
    }
}
pub trait Cont: Continuous<f64, f64> + ContinuousCDF<f64, f64> {}
impl Cont for Normal {}
impl Cont for Gamma {}
impl Cont for Beta {}
impl Cont for Cauchy {}
impl Cont for ChiSquared {}
impl Cont for Exp {}
impl Cont for FisherSnedecor {}
impl Cont for Gumbel {}
impl Cont for InverseGamma {}
impl Cont for Laplace {}
impl Cont for LogNormal {}
impl Cont for Pareto {}
impl Cont for StudentsT {}
impl Cont for Triangular {}
impl Cont for Uniform {}
impl Cont for Weibull {}
