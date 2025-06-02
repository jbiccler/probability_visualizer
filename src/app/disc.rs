use egui::emath::Numeric;
use std::error::Error;
use strum::IntoEnumIterator;

use crate::distr::disc_distr::*;
use egui_plot::{Bar, BarChart, Plot};
use statrs::distribution::*;

use crate::distr::*;
pub struct DiscPanel {
    par1: f64,
    par2: f64,
    par3: f64,
    selected_distr: DistrTypes,
    defaults: Vec<MixedParam>,
}

impl Default for DiscPanel {
    fn default() -> Self {
        let defaults = DistrTypes::Poisson.get_defaults();
        let par1 = match defaults[0].clone() {
            MixedParam::Float { param: p } => p.default,
            _ => panic!(),
        };
        Self {
            par1,
            par2: 1.0,
            par3: 1.0,
            selected_distr: DistrTypes::Poisson,
            defaults,
        }
    }
}

impl DiscPanel {
    pub fn side_panel(&mut self, ui: &mut egui::Ui) {
        let mut reset = false;
        egui::ComboBox::from_label("Select a distribution")
            .selected_text(format!("{}", self.selected_distr))
            .height(1000.)
            .show_ui(ui, |ui| {
                for d in DistrTypes::iter() {
                    if ui
                        .selectable_value(&mut self.selected_distr, d.clone(), format!("{}", d))
                        .clicked()
                    {
                        reset = true;
                    }
                }
            });

        if reset {
            self.defaults = self.selected_distr.get_defaults();
            for (i, p) in self.defaults.iter().enumerate() {
                let par = match p {
                    MixedParam::_Signed { param: p } => p.default.to_f64(),
                    MixedParam::Unsigned { param: p } => p.default.to_f64(),
                    MixedParam::Float { param: p } => p.default,
                };
                if i == 0 {
                    self.par1 = par
                } else if i == 1 {
                    self.par2 = par
                } else if i == 2 {
                    self.par3 = par
                } else {
                    panic!()
                }
            }
        }

        ui.add_space(10.0);
        for (i, p) in self.defaults.clone().into_iter().enumerate() {
            match p {
                MixedParam::Unsigned { param: p } => {
                    ui.add(egui::Label::new(format!("{}:", p.name)));
                    if i == 0 {
                        ui.add(egui::DragValue::new(&mut self.par1).range(p.range));
                    } else if i == 1 {
                        ui.add(egui::DragValue::new(&mut self.par2).range(p.range));
                    } else if i == 2 {
                        ui.add(egui::DragValue::new(&mut self.par3).range(p.range));
                    } else {
                        panic!()
                    }
                }
                MixedParam::Float { param: p } => {
                    ui.add(egui::Label::new(format!("{}:", p.name)));
                    if p.name == "p" {
                        // probability parameter
                        if i == 0 {
                            ui.add(egui::Slider::new(&mut self.par1, p.range));
                        } else if i == 1 {
                            ui.add(egui::Slider::new(&mut self.par2, p.range));
                        } else if i == 2 {
                            ui.add(egui::Slider::new(&mut self.par3, p.range));
                        } else {
                            panic!()
                        }
                    } else if i == 0 {
                        ui.add(egui::DragValue::new(&mut self.par1).range(p.range));
                    } else if i == 1 {
                        ui.add(egui::DragValue::new(&mut self.par2).range(p.range));
                    } else if i == 2 {
                        ui.add(egui::DragValue::new(&mut self.par3).range(p.range));
                    } else {
                        panic!()
                    }
                }
                MixedParam::_Signed { param: p } => {
                    ui.add(egui::Label::new(format!("{}:", p.name)));
                    if i == 0 {
                        ui.add(egui::DragValue::new(&mut self.par1).range(p.range));
                    } else if i == 1 {
                        ui.add(egui::DragValue::new(&mut self.par2).range(p.range));
                    } else if i == 2 {
                        ui.add(egui::DragValue::new(&mut self.par3).range(p.range));
                    } else {
                        panic!()
                    }
                }
            }
        }
        ui.add_space(10.0);
    }

    pub fn central_panel(&mut self, ui: &mut egui::Ui) {
        let distr = self.get_distr();
        match distr {
            Err(_) => {}
            Ok(d) => {
                // Statrs inverse_cdf is not stable for discrete distributions
                let (min, max) = get_min_max(&d);
                let x = min..=max;
                let cdf_bars: Vec<Bar> = x
                    .clone()
                    .map(|x| Bar::new(x as f64, d.cdf(x)).name("CDF"))
                    .collect();

                let pmf_bars: Vec<Bar> = x
                    .map(|x| Bar::new(x as f64, d.pmf(x)).name("PMF"))
                    .collect();

                let cdf_chart = BarChart::new("CDF", cdf_bars);
                let pmf_chart = BarChart::new("PMF", pmf_bars);

                Plot::new("MainPlot").view_aspect(2.0).show(ui, |plot_ui| {
                    plot_ui.bar_chart(pmf_chart);
                    plot_ui.bar_chart(cdf_chart);
                    plot_ui.set_plot_bounds(egui_plot::PlotBounds::from_min_max(
                        [min as f64 - 1., 0.0],
                        [max as f64 + 1., 1.2],
                    ));
                    // allow x axis to use auto bounds
                    plot_ui.set_auto_bounds(egui::Vec2b::new(false, true))
                });
            }
        }
    }
}

impl DiscPanel {
    fn get_distr(&self) -> Result<Box<dyn Disc>, Box<dyn Error>> {
        let res: Box<dyn Disc> = match self.selected_distr {
            DistrTypes::Poisson => Box::new(Poisson::new(self.par1)?),
            DistrTypes::Binomial => Box::new(Binomial::new(self.par1, self.par2 as u64)?),
            DistrTypes::Bernoulli => Box::new(Bernoulli::new(self.par1)?),
            DistrTypes::Geometric => Box::new(Geometric::new(self.par1)?),
            DistrTypes::Hypergeometric => Box::new(Hypergeometric::new(
                self.par1 as u64,
                self.par2 as u64,
                self.par3 as u64,
            )?),
            DistrTypes::NegativeBinomial => Box::new(NegativeBinomial::new(self.par1, self.par2)?),
        };
        Ok(res)
    }
}

fn get_min_max(distr: &Box<dyn Disc>) -> (u64, u64) {
    let mut min = 0;
    while distr.cdf(min) < 0.001 {
        min += 1
    }
    min = min.saturating_sub(1);
    let mut max = min + 1;
    while distr.cdf(max) < 0.999 && max < u64::MAX {
        max += 1;
    }
    (min, max)
}
