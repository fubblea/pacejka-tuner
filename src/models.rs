use egui::Ui;

pub trait Model {
    fn calc_f(&self, slip: f64) -> f64;
    fn create_sliders(&mut self, ui: &mut egui::Ui);
    fn get_model_axis_label(&self) -> String;
    fn get_model_label(&self) -> String;
}

#[derive(serde::Deserialize, serde::Serialize, Debug, PartialEq)]
pub enum PacejkaModel {
    LateralSimple(Simple),
    Lateral94(Lateral),
    Longitudinal94(Longitudinal),
}

impl Model for PacejkaModel {
    fn calc_f(&self, slip: f64) -> f64 {
        match self {
            Self::LateralSimple(v) => v.calc_f(slip),
            Self::Lateral94(v) => v.calc_f(slip),
            Self::Longitudinal94(v) => v.calc_f(slip),
        }
    }

    fn create_sliders(&mut self, ui: &mut egui::Ui) {
        match self {
            Self::LateralSimple(v) => v.create_sliders(ui),
            Self::Lateral94(v) => v.create_sliders(ui),
            Self::Longitudinal94(v) => v.create_sliders(ui),
        }
    }

    fn get_model_axis_label(&self) -> String {
        match self {
            Self::LateralSimple(v) => v.get_model_axis_label(),
            Self::Lateral94(v) => v.get_model_axis_label(),
            Self::Longitudinal94(v) => v.get_model_axis_label(),
        }
    }

    fn get_model_label(&self) -> String {
        match self {
            Self::LateralSimple(v) => v.get_model_label(),
            Self::Lateral94(v) => v.get_model_label(),
            Self::Longitudinal94(v) => v.get_model_label(),
        }
    }
}

#[derive(serde::Deserialize, serde::Serialize, Debug, PartialEq)]
struct Simple {
    fz: f64, // N
    b: f64,
    c: f64,
    d: f64,
    e: f64,
}

impl Default for Simple {
    fn default() -> Self {
        Self {
            fz: 1000.,
            b: 10.,
            c: 1.9,
            d: 1.,
            e: 0.97,
        }
    }
}

impl Model for Simple {
    fn calc_f(&self, slip: f64) -> f64 {
        self.fz
            * self.d
            * (self.c * (self.b * slip - self.e * (self.b * slip - (self.b * slip).atan())).atan())
                .sin()
    }

    fn create_sliders(&mut self, ui: &mut egui::Ui) {
        ui.columns(2, |columns| {
            columns[0].add(egui::Slider::new(&mut self.fz, 0.0..=100000.0).text("Fz [N]"));
            columns[0].add(egui::Slider::new(&mut self.b, 0.0..=20.0).text("B (Stiffness)"));
            columns[0].add(egui::Slider::new(&mut self.c, 0.0..=10.0).text("C (Shape)"));
            columns[1].add(egui::Slider::new(&mut self.d, 0.0..=5.0).text("D (Peak)"));
            columns[1].add(egui::Slider::new(&mut self.e, -10.0..=10.0).text("E (Curvature)"));
        });
    }

    fn get_model_axis_label(&self) -> String {
        String::from("Lateral Force (N)")
    }

    fn get_model_label(&self) -> String {
        String::from("Simple")
    }
}

#[derive(serde::Deserialize, serde::Serialize, Debug, PartialEq)]
struct Lateral {
    fz: f64, // N
}

impl Default for Lateral {
    fn default() -> Self {
        todo!()
    }
}

impl Model for Lateral {
    fn calc_f(&self, slip: f64) -> f64 {
        todo!()
    }

    fn create_sliders(&mut self, ui: &mut egui::Ui) {
        todo!()
    }

    fn get_model_axis_label(&self) -> String {
        todo!()
    }

    fn get_model_label(&self) -> String {
        todo!()
    }
}

#[derive(serde::Deserialize, serde::Serialize, Debug, PartialEq)]
struct Longitudinal {
    fz: f64, // N
}

impl Default for Longitudinal {
    fn default() -> Self {
        todo!()
    }
}

impl Model for Longitudinal {
    fn calc_f(&self, slip: f64) -> f64 {
        todo!()
    }

    fn create_sliders(&mut self, ui: &mut egui::Ui) {
        todo!()
    }

    fn get_model_axis_label(&self) -> String {
        todo!()
    }

    fn get_model_label(&self) -> String {
        todo!()
    }
}

pub enum ModelType {
    LateralSimple,
    Lateral94,
    Longitudinal94,
}

pub fn create_model(model_name: ModelType) -> PacejkaModel {
    match model_name {
        ModelType::LateralSimple => PacejkaModel::LateralSimple(Simple::default()),
        ModelType::Lateral94 => PacejkaModel::Lateral94(Lateral::default()),
        ModelType::Longitudinal94 => PacejkaModel::Longitudinal94(Longitudinal::default()),
    }
}
