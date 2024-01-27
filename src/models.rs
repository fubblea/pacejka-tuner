use egui::Ui;

#[derive(serde::Deserialize, serde::Serialize, Debug, PartialEq)]
pub enum PacejkaModels {
    Simple(Simple),
    Lateral,
    Longitudinal,
}

#[derive(serde::Deserialize, serde::Serialize, Debug, PartialEq)]
pub struct Simple {
    pub label: String,
    fz: f64, // N
    b: f64,
    c: f64,
    d: f64,
    e: f64,
}

impl Simple {
    pub fn default() -> Self {
        Self {
            label: String::from("Simple"),
            fz: 1000.,
            b: 10.,
            c: 1.9,
            d: 1.,
            e: 0.97,
        }
    }

    pub fn calc_f(&self, slip: f64) -> f64 {
        self.fz
            * self.d
            * (self.c * (self.b * slip - self.e * (self.b * slip - (self.b * slip).atan())).atan())
                .sin()
    }

    pub fn create_sliders(&mut self, ui: &mut egui::Ui) {
        ui.columns(2, |columns| {
            columns[0].add(egui::Slider::new(&mut self.fz, 0.0..=100000.0).text("Fz [N]"));
            columns[0].add(egui::Slider::new(&mut self.b, 4.0..=12.0).text("B (Stiffness)"));
            columns[0].add(egui::Slider::new(&mut self.c, 1.0..=2.0).text("C (Shape)"));
            columns[1].add(egui::Slider::new(&mut self.d, 0.1..=0.9).text("D (Peak)"));
            columns[1].add(egui::Slider::new(&mut self.e, -10.0..=1.0).text("E (Curvature)"));
        });
    }
}
