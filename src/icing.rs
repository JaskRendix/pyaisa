use crate::thermo::{saturation_vapor_pressure, vapor_pressure};

/// Liquid water content (LWC) approximation (g/m³)
pub fn lwc(t: f64, rh: f64) -> f64 {
    let e = vapor_pressure(t, rh);
    let es = saturation_vapor_pressure(t);
    let ratio = (e / es).min(1.0);
    0.5 * ratio // crude but useful
}

/// Supercooled fraction
pub fn supercooled_fraction(t: f64) -> f64 {
    if t >= 273.15 {
        0.0
    } else if t <= 253.15 {
        1.0
    } else {
        (273.15 - t) / 20.0
    }
}

/// Icing severity index (0–1)
pub fn icing_severity(t: f64, rh: f64) -> f64 {
    let lwc_val = lwc(t, rh);
    let sc = supercooled_fraction(t);
    (lwc_val * sc).min(1.0)
}

/// Freezing fraction of droplets
pub fn freezing_fraction(t: f64) -> f64 {
    supercooled_fraction(t)
}
