use super::linspace;
use crate::distr::cont_distr::DistrTypes;
use egui::emath::Numeric;
use std::error::Error;
use strum::IntoEnumIterator;

use egui_plot::{Line, Plot, PlotPoints};
use statrs::distribution::*;

use super::show_summary_stats_table;
use crate::distr::cont_distr::*;
use crate::distr::*;

pub struct ContPanel {
    par1: f64,
    par2: f64,
    par3: f64,
    selected_distr: DistrTypes,
    defaults: Vec<MixedParam>,
}

impl ContPanel {
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
        ui.separator();

        ui.heading("Summary Statistics:");
        ui.add_space(10.0);
        // Summary statistics
        let distr = self.get_distr();
        if let Ok(d) = distr {
            let summary = SummaryStats::new(&(*d));
            show_summary_stats_table(ui, &summary);
            ui.add_space(10.0);
        }
    }
    pub fn central_panel(&mut self, ui: &mut egui::Ui) {
        let distr = self.get_distr();
        match distr {
            Err(_) => {}
            Ok(d) => {
                let min = d.inverse_cdf(0.0001);
                let max = d.inverse_cdf(0.9999);
                if min != -f64::INFINITY && max != f64::INFINITY {
                    // CDF
                    let x = linspace::<f64>(min, max, 1000);
                    let cdf_points: PlotPoints<'_> = x.clone().map(|x| [x, d.cdf(x)]).collect();
                    let cdf = Line::new("Default", cdf_points).name("CDF");

                    let pdfs: Vec<f64> = x.clone().map(|x| d.pdf(x)).collect();
                    // Get y axis upper bound for graph
                    let mut max_y = 1.0;
                    for &p in pdfs.iter() {
                        if p > max_y {
                            max_y = p;
                        }
                    }
                    max_y += 0.2;
                    let pdf_points: PlotPoints<'_> = x.zip(pdfs).map(|(x, y)| [x, y]).collect();
                    let pdf = Line::new("Default", pdf_points).name("PDF");
                    Plot::new("MainPlot").view_aspect(2.0).show(ui, |plot_ui| {
                        plot_ui.line(pdf);
                        plot_ui.line(cdf);
                        // allow x axis to use auto bounds
                        plot_ui.set_plot_bounds(egui_plot::PlotBounds::from_min_max(
                            [min - 1., 0.0],
                            [max + 1., max_y],
                        ));
                    });
                }
            }
        }
    }
}

impl Default for ContPanel {
    fn default() -> Self {
        let defaults = DistrTypes::Normal.get_defaults();
        let par1 = match defaults[0].clone() {
            MixedParam::Float { param: p } => p.default,
            _ => panic!(),
        };
        let par2 = match defaults[1].clone() {
            MixedParam::Float { param: p } => p.default,
            _ => panic!(),
        };
        Self {
            par1,
            par2,
            par3: 0.0,
            selected_distr: DistrTypes::Normal,
            defaults,
        }
    }
}

impl ContPanel {
    fn get_distr(&self) -> Result<Box<dyn Cont>, Box<dyn Error>> {
        let res: Box<dyn Cont> = match self.selected_distr {
            DistrTypes::Normal => Box::new(Normal::new(self.par1, self.par2)?),
            DistrTypes::Gamma => Box::new(Gamma::new(self.par1, self.par2)?),
            DistrTypes::Beta => Box::new(Beta::new(self.par1, self.par2)?),
            DistrTypes::Cauchy => Box::new(Cauchy::new(self.par1, self.par2)?),
            DistrTypes::ChiSquared => Box::new(ChiSquared::new(self.par1)?),
            DistrTypes::Exp => Box::new(Exp::new(self.par1)?),
            DistrTypes::FisherSnedecor => Box::new(FisherSnedecor::new(self.par1, self.par2)?),
            DistrTypes::Gumbel => Box::new(Gumbel::new(self.par1, self.par2)?),
            DistrTypes::InverseGamma => Box::new(InverseGamma::new(self.par1, self.par2)?),
            DistrTypes::Laplace => Box::new(Laplace::new(self.par1, self.par2)?),
            DistrTypes::LogNormal => Box::new(LogNormal::new(self.par1, self.par2)?),
            DistrTypes::Pareto => Box::new(Pareto::new(self.par1, self.par2)?),
            DistrTypes::StudentsT => Box::new(StudentsT::new(self.par1, self.par2, self.par3)?),
            DistrTypes::Triangular => Box::new(Triangular::new(self.par1, self.par2, self.par3)?),
            DistrTypes::Uniform => Box::new(Uniform::new(self.par1, self.par2)?),
            DistrTypes::Weibull => Box::new(Weibull::new(self.par1, self.par2)?),
        };
        Ok(res)
    }
}
