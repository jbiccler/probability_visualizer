use super::{BARELY_POSITIVE, MixedParam, Param};
use statrs::distribution::*;
use std::{fmt::Display, ops::RangeInclusive};
use strum_macros::EnumIter;

#[derive(Debug, PartialEq, PartialOrd, EnumIter, Clone)]
pub enum DistrTypes {
    Poisson,
    Binomial,
}

impl DistrTypes {
    pub fn get_defaults(&self) -> Vec<MixedParam> {
        match self {
            DistrTypes::Poisson => vec![MixedParam::Float {
                param: Param {
                    default: 0_f64,
                    range: RangeInclusive::new(BARELY_POSITIVE, f64::MAX),
                    name: "Lambda".to_owned(),
                },
            }],
            DistrTypes::Binomial => vec![
                MixedParam::Float {
                    param: Param {
                        default: 0.5_f64,
                        range: RangeInclusive::new(BARELY_POSITIVE, f64::MAX),
                        name: "Shape".to_owned(),
                    },
                },
                MixedParam::Unsigned {
                    param: Param {
                        default: 5_u64,
                        range: RangeInclusive::new(0, u64::MAX),
                        name: "Scale".to_owned(),
                    },
                },
            ],
        }
    }
}

pub trait Disc: Discrete<u64, f64> + DiscreteCDF<u64, f64> {}
impl Disc for Binomial {}
impl Disc for Poisson {}
