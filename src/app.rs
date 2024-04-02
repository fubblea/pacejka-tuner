use crate::models::{create_model, Model, ModelType, PacejkaModel};
use egui_plot::{Line, Plot, PlotPoints};

/// We derive Deserialize/Serialize so we can persist app state on shutdown.
#[derive(serde::Deserialize, serde::Serialize)]
#[serde(default)] // if we add new fields, give them default values when deserializing old state
pub struct TunerApp {
    model: PacejkaModel,
}

impl Default for TunerApp {
    fn default() -> Self {
        Self {
            model: create_model(ModelType::LateralSimple),
        }
    }
}

impl TunerApp {
    /// Called once before the first frame.
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        // This is also where you can customize the look and feel of egui using
        // `cc.egui_ctx.set_visuals` and `cc.egui_ctx.set_fonts`.

        // Load previous app state (if any).
        // Note that you must enable the `persistence` feature for this to work.
        if let Some(storage) = cc.storage {
            return eframe::get_value(storage, eframe::APP_KEY).unwrap_or_default();
        }

        Default::default()
    }
}

impl eframe::App for TunerApp {
    /// Called by the frame work to save state before shutdown.
    fn save(&mut self, storage: &mut dyn eframe::Storage) {
        eframe::set_value(storage, eframe::APP_KEY, self);
    }

    /// Called each time the UI needs repainting, which may be many times per second.
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // Put your widgets into a `SidePanel`, `TopBottomPanel`, `CentralPanel`, `Window` or `Area`.
        // For inspiration and more examples, go to https://emilk.github.io/egui

        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            // The top panel is often a good place for a menu bar:

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

                egui::widgets::global_dark_light_mode_buttons(ui);
            });
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            // The central panel the region left after adding TopPanel's and SidePanel's
            ui.heading("Pacejka Tuner");

            let label = &self.model.get_model_label();

            egui::ComboBox::from_label("Select a model!")
                .selected_text((label).to_string())
                .show_ui(ui, |ui| {
                    ui.selectable_value(
                        &mut self.model,
                        create_model(ModelType::LateralSimple),
                        "Lateral Simple",
                    );
                    ui.selectable_value(
                        &mut self.model,
                        create_model(ModelType::Lateral94),
                        "Lateral '94",
                    );
                    ui.selectable_value(
                        &mut self.model,
                        create_model(ModelType::Longitudinal94),
                        "Longitudinal '94",
                    );
                });

            ui.spacing();

            let model_plot: PlotPoints = (0..1600)
                .step_by(1)
                .map(|i| {
                    let x = i as f64 * 0.001;
                    [x, self.model.calc_f(x)]
                })
                .collect();
            let line = Line::new(model_plot);
            Plot::new("model_plot")
                .view_aspect(2.0)
                .y_axis_label(self.model.get_model_axis_label())
                .x_axis_label("Slip Angle (rad)")
                .show(ui, |plot_ui| plot_ui.line(line));

            self.model.create_sliders(ui)
        });
    }
}
