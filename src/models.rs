pub trait Model {
    fn calc_f(&self, slip: f64) -> f64;
    fn create_sliders(&mut self, ui: &mut egui::Ui);
    fn get_model_axis_label(&self) -> String;
    fn get_model_label(&self) -> String;
}

pub enum ModelType {
    LateralSimple,
    Lateral94,
    Longitudinal94,
}

#[derive(serde::Deserialize, serde::Serialize, Debug, PartialEq)]
pub enum PacejkaModel {
    LateralSimple(Simple),
    Lateral94(Lateral),
    Longitudinal94(Longitudinal),
}

impl PacejkaModel {
    pub fn create_model(model_name: ModelType) -> Self {
        match model_name {
            ModelType::LateralSimple => Self::LateralSimple(Simple::default()),
            ModelType::Lateral94 => Self::Lateral94(Lateral::default()),
            ModelType::Longitudinal94 => Self::Longitudinal94(Longitudinal::default()),
        }
    }
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
pub struct Simple {
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
pub struct Lateral {
    fz: f64, // N
    a0: f64,
    a1: f64,
    a2: f64,
    a3: f64,
    a4: f64,
    a5: f64,
    a6: f64,
    a7: f64,
    a8: f64,
    a9: f64,
    a10: f64,
    a11: f64,
    a12: f64,
    a13: f64,
    a14: f64,
    a15: f64,
    a16: f64,
    a17: f64,
}

impl Default for Lateral {
    fn default() -> Self {
        Self {
            fz: 1000., // N
            a0: 1.4,
            a1: 0.,
            a2: 1100.,
            a3: 1100.,
            a4: 10.,
            a5: 0.,
            a6: 0.,
            a7: -2.,
            a8: 0.,
            a9: 0.,
            a10: 0.,
            a11: 0.,
            a12: 0.,
            a13: 0.,
            a14: 0.,
            a15: 0.,
            a16: 0.,
            a17: 0.,
        }
    }
}

impl Model for Lateral {
    fn calc_f(&self, slip: f64) -> f64 {
        let c = self.a0;
        let d = self.fz * (self.a1 * self.fz + self.a2) * (1. - self.a15 * slip.powf(2.));
        let bcd = self.a3 * ((self.fz / self.a4).atan() * 2.).sin() * (1. - self.a5 * slip.abs());
        let b = bcd / (c * d);
        let h = self.a8 * self.fz + self.a9 + self.a10 * slip;
        let e = (self.a6 * self.fz + self.a7)
            * (1. - (self.a16 * slip + self.a17) * (slip + h).signum());
        let v = self.a11 * self.fz + self.a12 + (self.a13 * self.fz + self.a14) * slip * self.fz;
        let bx1 = b * (slip + h);

        d * (c * (bx1 - e * (bx1 - bx1.atan()).atan()).sin()) + v
    }

    fn create_sliders(&mut self, ui: &mut egui::Ui) {
        ui.columns(2, |columns| {
            columns[0].add(egui::Slider::new(&mut self.fz, 0.0..=100000.0).text("Fz [N]"));
            columns[0].add(egui::Slider::new(&mut self.a0, 0.0..=20.0).text("a0 (Shape Factor)"));
            columns[0].add(
                egui::Slider::new(&mut self.a1, -100.0..=100.0)
                    .text("a1 (Load influence on lateral mu) [1/kN]"),
            );
            columns[0].add(egui::Slider::new(&mut self.a2, 500.0..=1000.0).text("a2 (Lateral mu)"));
            columns[0].add(
                egui::Slider::new(&mut self.a3, 100.0..=2000.0)
                    .text("a3 (Stiffness with slip) [N/deg]"),
            );
            columns[0].add(
                egui::Slider::new(&mut self.a4, 0.0..=100.0)
                    .text("a4 (ddy of stiffness / load) [1/KN]"),
            );
            columns[0].add(
                egui::Slider::new(&mut self.a5, -0.1..=0.1)
                    .text("a5 (Camber influence on stiffness) [%/deg/100]"),
            );
            columns[0].add(
                egui::Slider::new(&mut self.a6, -5.0..=5.0).text("a6 (Curvature change with load)"),
            );
            columns[0]
                .add(egui::Slider::new(&mut self.a7, -20.0..=5.0).text("a7 (Curvature factor)"));
            columns[0].add(
                egui::Slider::new(&mut self.a8, -5.0..=5.0)
                    .text("a8 (Load influence on horizontal shift) [deg/kN]"),
            );
            columns[1].add(
                egui::Slider::new(&mut self.a9, -5.0..=5.0)
                    .text("a9 (Horizontal shift at no load) [deg]"),
            );
            columns[1].add(
                egui::Slider::new(&mut self.a10, -1.0..=1.0)
                    .text("a10 (Camber influence on horizontal shift) [deg/deg]"),
            );
            columns[1].add(
                egui::Slider::new(&mut self.a11, -500.0..=500.0).text("a11 (Vertical shift) [N]"),
            );
            columns[1].add(
                egui::Slider::new(&mut self.a12, -10.0..=10.0)
                    .text("a12 (Vertical shift at no load) [N]"),
            );
            columns[1].add(
                egui::Slider::new(&mut self.a13, -10.0..=10.0)
                    .text("a13 (Camber influence on vertical shift, loaded) [N/deg/kN]"),
            );
            columns[1].add(
                egui::Slider::new(&mut self.a14, -20.0..=20.0)
                    .text("a14 (Camber influence on vertical shift) [N/deg]"),
            );
            columns[1].add(
                egui::Slider::new(&mut self.a15, -1.0..=1.0)
                    .text("a15 (Camber influence on lateral friction coefficient) [1/deg]"),
            );
            columns[1].add(
                egui::Slider::new(&mut self.a16, -1.0..=1.0)
                    .text("a16 (Curvature change with camber)"),
            );
            columns[1].add(
                egui::Slider::new(&mut self.a17, -5.0..=5.0)
                    .text("a17 (Curvature change with camber)"),
            );
        });
    }

    fn get_model_axis_label(&self) -> String {
        String::from("Lateral Force (N)")
    }

    fn get_model_label(&self) -> String {
        String::from("Lateral '94")
    }
}

#[derive(serde::Deserialize, serde::Serialize, Debug, PartialEq)]
pub struct Longitudinal {
    fz: f64, // N
    b0: f64,
    b1: f64,
    b2: f64,
    b3: f64,
    b4: f64,
    b5: f64,
    b6: f64,
    b7: f64,
    b8: f64,
    b9: f64,
    b10: f64,
    b11: f64,
    b12: f64,
    b13: f64,
}

impl Default for Longitudinal {
    fn default() -> Self {
        Self {
            fz: 1000., // N
            b0: 1.5,
            b1: 0.,
            b2: 1100.,
            b3: 0.,
            b4: 300.,
            b5: 0.,
            b6: 0.,
            b7: 0.,
            b8: -2.,
            b9: 0.,
            b10: 0.,
            b11: 0.,
            b12: 0.,
            b13: 0.,
        }
    }
}

impl Model for Longitudinal {
    fn calc_f(&self, slip: f64) -> f64 {
        let c = self.b0;
        let d = self.fz * (self.b1 * self.fz + self.b2);
        let bcd = (self.b3 * self.fz.powf(2.) + self.b4 * self.fz) * (-self.b5 * self.fz).exp();
        let b = bcd / (c * d);
        let h = self.b9 * self.fz + self.b10;
        let e = (self.b6 * self.fz.powf(2.) + self.b7 * self.fz + self.b8)
            * (1.0 - self.b13 * (slip + h).signum());
        let v = self.b11 * self.fz + self.b12;
        let bx1 = b * (slip * h);

        d * (c * (bx1 - e * (bx1 - bx1.atan())).atan()).sin() + v
    }

    fn create_sliders(&mut self, ui: &mut egui::Ui) {
        ui.columns(2, |columns| {
            columns[0].add(egui::Slider::new(&mut self.fz, 0.0..=100000.0).text("Fz [N]"));
            columns[0].add(egui::Slider::new(&mut self.b0, 0.0..=2.0).text("b0 (Shape factor)"));
            columns[0].add(
                egui::Slider::new(&mut self.b1, -100.0..=100.0)
                    .text("b1 (Load influence on mu) [1/kN]"),
            );
            columns[0]
                .add(egui::Slider::new(&mut self.b2, 500.0..=2000.0).text("b2 (Longitudinal mu)"));
            columns[0].add(
                egui::Slider::new(&mut self.b3, -100.0..=100.0)
                    .text("b3 (Curvature factor of stiffness/load) [N/%/kN^2]"),
            );
            columns[0].add(
                egui::Slider::new(&mut self.b4, 500.0..=600.0)
                    .text("b4 (Change of stiffness with slip) [N/%]"),
            );
            columns[0].add(
                egui::Slider::new(&mut self.b5, -5.0..=5.0)
                    .text("b5 (Change of progressivity of stiffness/load) [1/kN]"),
            );
            columns[0].add(
                egui::Slider::new(&mut self.b6, -1.0..=1.0)
                    .text("b6 (Curvature change with load^2)"),
            );
            columns[1].add(
                egui::Slider::new(&mut self.b7, -10.0..=10.0)
                    .text("b7 (Curvature change with load)"),
            );
            columns[1]
                .add(egui::Slider::new(&mut self.b8, -50.0..=10.0).text("b8 (Curvature factor)"));
            columns[1].add(
                egui::Slider::new(&mut self.b9, -5.0..=5.0)
                    .text("b9 (Load influence on horizontal shift) [%/kN]"),
            );
            columns[1].add(
                egui::Slider::new(&mut self.b10, -10.0..=10.0).text("b10 (Horizontal shift) [%]"),
            );
            columns[1].add(
                egui::Slider::new(&mut self.b11, -500.0..=500.0).text("b11 (Vertical shift) [N]"),
            );
            columns[1].add(
                egui::Slider::new(&mut self.b12, -100.0..=100.0)
                    .text("b12 (Vertical shift at load = 0) [N]"),
            );
            columns[1]
                .add(egui::Slider::new(&mut self.b13, -10.0..=10.0).text("b13 (Curvature shift)"));
        })
    }

    fn get_model_axis_label(&self) -> String {
        String::from("Longitudinal Force (N)")
    }

    fn get_model_label(&self) -> String {
        String::from("Longitudinal '94")
    }
}
