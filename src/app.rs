use cont::*;
use disc::*;

mod cont;
mod disc;

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
    cont_panel: ContPanel,
    disc_panel: DiscPanel,
    open_panel: Panel,
}

impl Default for TemplateApp {
    fn default() -> Self {
        Self {
            cont_panel: ContPanel::default(),
            disc_panel: DiscPanel::default(),
            open_panel: Panel::Cont,
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
                ui.horizontal_wrapped(|ui| {
                    let container_response = ui.response();
                    container_response.widget_info(|| {
                        egui::WidgetInfo::labeled(egui::WidgetType::RadioGroup, true, "Select Demo")
                    });

                    ui.ctx()
                        .clone()
                        .with_accessibility_parent(container_response.id, || {
                            ui.selectable_value(&mut self.open_panel, Panel::Cont, "Continuous");
                            ui.selectable_value(&mut self.open_panel, Panel::Disc, "Discrete");
                        });
                });
                ui.heading("Probability distribution");
                ui.add_space(10.0);

                match self.open_panel {
                    Panel::Disc => self.disc_panel.side_panel(ui),
                    Panel::Cont => self.cont_panel.side_panel(ui),
                }
                ui.separator();
                ui.add_space(10.0);
                ui.image(egui::include_image!("../assets/Ferris.svg"));
            });

        egui::CentralPanel::default().show(ctx, |ui| {
            ui.vertical_centered(|ui| {
                ui.heading("Probability visualizer");
            });
            ui.separator();

            match self.open_panel {
                Panel::Disc => {
                    self.disc_panel.central_panel(ui);
                }
                Panel::Cont => {
                    self.cont_panel.central_panel(ui);
                }
            }

            ui.with_layout(egui::Layout::bottom_up(egui::Align::LEFT), |ui| {
                powered_by_egui_and_eframe(ui);
                egui::warn_if_debug_build(ui);
            });
        });
    }
}

fn linspace<T>(min: T, max: T, n: usize) -> impl Iterator<Item = T> + Clone
where
    T: Copy + Clone + From<f64> + Into<f64>,
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
