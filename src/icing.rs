use crate::thermo::{saturation_vapor_pressure, vapor_pressure};

/// Liquid water content (LWC) approximation (g/m³)
pub fn lwc(T: f64, rh: f64) -> f64 {
    let e = vapor_pressure(T, rh);
    let es = saturation_vapor_pressure(T);
    let ratio = (e / es).min(1.0);
    0.5 * ratio // crude but useful
}

/// Supercooled fraction
pub fn supercooled_fraction(T: f64) -> f64 {
    if T >= 273.15 {
        0.0
    } else if T <= 253.15 {
        1.0
    } else {
        (273.15 - T) / 20.0
    }
}

/// Icing severity index (0–1)
pub fn icing_severity(T: f64, rh: f64) -> f64 {
    let lwc_val = lwc(T, rh);
    let sc = supercooled_fraction(T);
    (lwc_val * sc).min(1.0)
}

/// Freezing fraction of droplets
pub fn freezing_fraction(T: f64) -> f64 {
    supercooled_fraction(T)
}
