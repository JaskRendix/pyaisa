/// Potential temperature (dry)
/// θ = T * (p0 / p)^(R/cp)
pub fn potential_temperature(t: f64, p: f64) -> f64 {
    const P0: f64 = 100000.0;
    const R: f64 = 287.05287;
    const CP: f64 = 1004.0;
    t * (P0 / p).powf(R / CP)
}

/// Moist adiabatic lapse rate (approx)
/// Γ_m = g * (1 + L*q/(R*T)) / (cp + L^2*q/(R*T^2))
pub fn moist_lapse_rate(t: f64, p: f64, rh: f64) -> f64 {
    const G: f64 = 9.80665;
    const R: f64 = 287.05287;
    const CP: f64 = 1004.0;
    const L: f64 = 2.5e6;

    let e = vapor_pressure(t, rh);
    let q = mixing_ratio(p, e);

    let num = G * (1.0 + (L * q) / (R * t));
    let den = CP + (L * L * q) / (R * t * t);

    num / den
}

/// Wet-bulb temperature (Stull 2011 approximation)
pub fn wet_bulb_temperature(t: f64, rh: f64) -> f64 {
    let t_c = t - 273.15;
    let tw = t_c * f64::atan(0.151977 * (rh + 8.313659).sqrt()) + f64::atan(t_c + rh)
        - f64::atan(rh - 1.676331)
        + 0.00391838 * rh.powf(1.5) * f64::atan(0.023101 * rh)
        - 4.686035;

    tw + 273.15
}

/// Saturation vapor pressure over water (Magnus formula) [Pa]
/// Valid for -45°C to +60°C (228.15 K to 333.15 K)
pub fn saturation_vapor_pressure(t: f64) -> f64 {
    let tc = t - 273.15;
    let tc_clamped = tc.clamp(-45.0, 60.0);
    6.112 * (17.67 * tc_clamped / (tc_clamped + 243.5)).exp() * 100.0
}

/// Actual vapor pressure from relative humidity (0–1) [Pa]
pub fn vapor_pressure(t: f64, rh: f64) -> f64 {
    rh.clamp(0.0, 1.0) * saturation_vapor_pressure(t)
}

/// Mixing ratio [kg/kg]
pub fn mixing_ratio(p: f64, e: f64) -> f64 {
    if e >= p {
        return f64::NAN;
    }
    0.622 * e / (p - e)
}

/// Dew point temperature [K]
pub fn dew_point(e: f64) -> f64 {
    let eh = e / 100.0; // Pa → hPa
    if eh <= 0.0 {
        return f64::NAN;
    }
    let ln = (eh / 6.112).ln();
    let td_c = (243.5 * ln) / (17.67 - ln);
    td_c + 273.15
}

/// Virtual temperature [K]
pub fn virtual_temperature(t: f64, w: f64) -> f64 {
    t * (1.0 + 0.61 * w)
}

/// Moist-air density [kg/m³]
pub fn moist_air_density(p: f64, t: f64, rh: f64) -> f64 {
    const RD: f64 = 287.05287;
    let e = vapor_pressure(t, rh);
    if e >= p {
        return f64::NAN;
    }
    let w = 0.622 * e / (p - e);
    let r_m = RD * (1.0 + 1.6078 * w);
    p / (r_m * t)
}

/// Moist-air speed of sound [m/s]
pub fn moist_speed_of_sound(t: f64, rh: f64, p: f64) -> f64 {
    const RD: f64 = 287.05287;
    const CPD: f64 = 1004.685;
    const CPV: f64 = 1859.0;

    let e = vapor_pressure(t, rh);
    if e >= p {
        return f64::NAN;
    }
    let w = 0.622 * e / (p - e);

    let r_m = RD * (1.0 + 1.6078 * w);
    let c_p = CPD + w * CPV;
    let c_v = c_p - r_m;
    let gamma_m = c_p / c_v;

    (gamma_m * r_m * t).sqrt()
}
