use super::Model;

pub fn get_ml(num: f64, model: &Model) -> f64 {
    (&model.bottleSize * num) / 100.0
}

pub fn get_pct(num: f64, model: &Model) -> f64 {
    (num / &model.bottleSize) * 100.0
}

pub enum Material {
    PG, VG, Nic,
}

pub fn get_grams(num: f64, material: Material) -> f64 {
    let grams = match material {
        Material::PG => 1.038,
        Material::VG => 1.26,
        Material::Nic => 1.01,
    };

    num * grams
}

pub fn get_num(s: String) -> f64 {
    match s.parse::<f64>() {
        Ok(v) => v,
        _ => 0.0,
    }
}
