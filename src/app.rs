use egui::emath::Numeric;
use std::error::Error;
use strum::IntoEnumIterator;

use egui_plot::{Line, Plot, PlotPoints};
use statrs::distribution::*;

use crate::distr::cont::*;
use crate::distr::*;

#[derive(PartialEq)]
enum Panel {
    Cont,
    Disc,
}

impl Default for Panel {
    fn default() -> Self {
        Self::Cont
    }
}

pub struct TemplateApp {
    par1: f64,
    par2: f64,
    par3: f64,
    selected_distr: DistrTypes,
    defaults: Vec<MixedParam>,
}

impl Default for TemplateApp {
    fn default() -> Self {
        Self {
            par1: 0.0,
            par2: 1.0,
            par3: 1.0,
            selected_distr: DistrTypes::Normal,
            defaults: DistrTypes::Normal.get_defaults(),
        }
    }
}

impl TemplateApp {
    pub fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        Default::default()
    }
}

impl eframe::App for TemplateApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        catppuccin_egui::set_theme(ctx, catppuccin_egui::MOCHA);
        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            egui::menu::bar(ui, |ui| {
                // NOTE: no File->Quit on web pages!
                let is_web = cfg!(target_arch = "wasm32");
                if !is_web {
                    ui.menu_button("File", |ui| {
                        if ui.button("Quit").clicked() {
                            ctx.send_viewport_cmd(egui::ViewportCommand::Close);
                        }
                    });
                    ui.add_space(16.0);
                }
            });
        });
        egui::SidePanel::left("left_panel")
            .min_width(200.0)
            .show(ctx, |ui| {
                ui.heading("Probability distribution");
                ui.add_space(10.0);

                let mut reset = false;
                egui::ComboBox::from_label("Select a distribution")
                    .selected_text(format!("{}", self.selected_distr))
                    .height(1000.)
                    .show_ui(ui, |ui| {
                        for d in DistrTypes::iter() {
                            if ui
                                .selectable_value(
                                    &mut self.selected_distr,
                                    d.clone(),
                                    format!("{}", d),
                                )
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
                            MixedParam::Signed { param: p } => p.default.to_f64(),
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
                    // match self.defaults.len() {
                    //     1 => self.par1 = self.defaults[0].param.default,
                    //     2 => {
                    //         self.par1 = self.defaults[0].param.default;
                    //         self.par2 = self.defaults[1].aram.default;
                    //     }
                    //     3 => {
                    //         self.par1 = self.defaults[0].param.default;
                    //         self.par2 = self.defaults[1].param.default;
                    //         self.par3 = self.defaults[2].param.default;
                    //     }
                    //     _ => panic!(),
                    // }
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
                        MixedParam::Signed { param: p } => {
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
                    // ui.add(egui::Label::new(format!("{}:", p.param.name)));
                    // ui.add(egui::DragValue::new(&mut self.par1).range(p.range));
                }
                ui.add_space(10.0);
                ui.separator();
                ui.add_space(10.0);
                ui.image(egui::include_image!("../assets/Ferris.svg"));
            });

        egui::CentralPanel::default().show(ctx, |ui| {
            ui.vertical_centered(|ui| {
                ui.heading("Probability visualizer");
            });
            ui.separator();

            let distr = self.get_cont_distr();
            match distr {
                Err(_) => {}
                Ok(d) => {
                    let min = d.inverse_cdf(0.0001);
                    let max = d.inverse_cdf(0.9999);
                    if min != -f64::INFINITY && max != f64::INFINITY {
                        let x = linspace::<f64>(min, max, 1000);
                        let cdf_points: PlotPoints<'_> = x.map(|x| [x, d.cdf(x)]).collect();
                        let cdf = Line::new("Default", cdf_points).name("CDF");

                        let x = linspace::<f64>(min, max, 1000);
                        let pdf_points: PlotPoints<'_> = x.map(|x| [x, d.pdf(x)]).collect();
                        let pdf = Line::new("Default", pdf_points).name("PDF");
                        Plot::new("MainPlot").view_aspect(2.0).show(ui, |plot_ui| {
                            plot_ui.line(cdf);
                            plot_ui.line(pdf)
                        });
                    }
                }
            }

            ui.with_layout(egui::Layout::bottom_up(egui::Align::LEFT), |ui| {
                powered_by_egui_and_eframe(ui);
                egui::warn_if_debug_build(ui);
            });
        });
    }
}

impl TemplateApp {
    fn get_cont_distr(&self) -> Result<Box<dyn Cont>, Box<dyn Error>> {
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

fn linspace<T>(min: T, max: T, n: usize) -> impl Iterator<Item = T>
where
    T: Copy + From<f64> + Into<f64>,
{
    let min_f = min.into();
    let max_f = max.into();

    let step = if n > 1 {
        (max_f - min_f) / (n as f64 - 1.0)
    } else {
        0.0
    };

    (0..n).map(move |i| T::from(min_f + step * i as f64))
}

fn powered_by_egui_and_eframe(ui: &mut egui::Ui) {
    ui.horizontal(|ui| {
        ui.spacing_mut().item_spacing.x = 0.0;
        ui.label("Powered by ");
        ui.hyperlink_to("egui", "https://github.com/emilk/egui");
        ui.label(" and ");
        ui.hyperlink_to(
            "eframe",
            "https://github.com/emilk/egui/tree/master/crates/eframe",
        );
        ui.label(".");
    });
}
