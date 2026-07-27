use crate::math::{mixing_ratio, saturation_vapor_pressure, vapor_pressure};

/// Potential temperature (dry)
/// θ = T * (p0 / p)^(R/cp)
pub fn potential_temperature(T: f64, p: f64) -> f64 {
    const P0: f64 = 100000.0;
    const R: f64 = 287.05287;
    const CP: f64 = 1004.0;
    T * (P0 / p).powf(R / CP)
}

/// Moist adiabatic lapse rate (approx)
/// Γ_m = g * (1 + L*q/(R*T)) / (cp + L^2*q/(R*T^2))
pub fn moist_lapse_rate(T: f64, p: f64, rh: f64) -> f64 {
    const G: f64 = 9.80665;
    const R: f64 = 287.05287;
    const CP: f64 = 1004.0;
    const L: f64 = 2.5e6;

    let e = vapor_pressure(T, rh);
    let q = mixing_ratio(p, e);

    let num = G * (1.0 + (L * q) / (R * T));
    let den = CP + (L * L * q) / (R * T * T);

    num / den
}

/// Wet-bulb temperature (Stull 2011 approximation)
pub fn wet_bulb_temperature(T: f64, rh: f64) -> f64 {
    let T_c = T - 273.15;
    let Tw = T_c * f64::atan(0.151977 * (rh + 8.313659).sqrt()) + f64::atan(T_c + rh)
        - f64::atan(rh - 1.676331)
        + 0.00391838 * rh.powf(1.5) * f64::atan(0.023101 * rh)
        - 4.686035;

    Tw + 273.15
}
