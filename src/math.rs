pub fn d(x: f64) -> f64 {
    (x.abs() < 1e-12) as u8 as f64
}

/// Convert geometric altitude h [m] → geopotential altitude H [m]
pub fn geometric_to_geopotential(h: f64) -> f64 {
    const RE: f64 = 6371000.0; // Earth radius [m]
    (RE * h) / (RE + h)
}

/// Convert geopotential altitude H [m] → geometric altitude h [m]
pub fn geopotential_to_geometric(H: f64) -> f64 {
    const RE: f64 = 6371000.0;
    if H >= RE {
        return f64::INFINITY; // or panic, or clamp
    }
    (RE * H) / (RE - H)
}

pub fn speed_of_sound(t: f64) -> f64 {
    const GAMMA: f64 = 1.4;
    const R: f64 = 287.05287;
    (GAMMA * R * t).sqrt()
}

pub fn dynamic_pressure(rho: f64, v: f64) -> f64 {
    0.5 * rho * v * v
}

pub fn mach(v: f64, a: f64) -> f64 {
    v / a
}

pub fn pressure_altitude(p: f64) -> f64 {
    const P0: f64 = 101325.0;
    const C: f64 = 44330.769; // SI-correct constant
    C * (1.0 - (p / P0).powf(0.190263))
}

pub fn density_altitude(p: f64, t: f64) -> f64 {
    let pa = pressure_altitude(p);
    let t_std = 288.15;
    pa + 118.8 * (t - t_std)
}

/// Saturation vapor pressure over water (Magnus formula)
/// Valid for -45°C to +60°C (228.15 K to 333.15 K)
pub fn saturation_vapor_pressure(t: f64) -> f64 {
    // Kelvin → Celsius
    let tc = t - 273.15;

    // Validity range check
    if tc < -45.0 || tc > 60.0 {
        // You can choose: clamp, warn, or return NaN
        // Here we clamp to the valid range for stability
        let tc_clamped = tc.clamp(-45.0, 60.0);
        let e = 6.112 * (17.67 * tc_clamped / (tc_clamped + 243.5)).exp() * 100.0;
        return e;
    }

    // Standard Magnus formula
    6.112 * (17.67 * tc / (tc + 243.5)).exp() * 100.0
}

/// Actual vapor pressure from RH (0–1)
pub fn vapor_pressure(t: f64, rh: f64) -> f64 {
    rh * saturation_vapor_pressure(t)
}

/// Mixing ratio (kg/kg)
pub fn mixing_ratio(p: f64, e: f64) -> f64 {
    // Prevent division by zero or negative denominator
    if e >= p {
        return f64::NAN; // or clamp, or return 0.0
    }

    0.622 * e / (p - e)
}

/// Dew point temperature (Kelvin)
pub fn dew_point(e: f64) -> f64 {
    let eh = e / 100.0; // Pa → hPa
    let ln = (eh / 6.112).ln();
    let td_c = (243.5 * ln) / (17.67 - ln);
    td_c + 273.15
}

/// Virtual temperature (Kelvin)
pub fn virtual_temperature(t: f64, w: f64) -> f64 {
    t * (1.0 + 0.61 * w)
}

/// Moist-air density (kg/m³)
pub fn moist_air_density(p: f64, t: f64, rh: f64) -> f64 {
    const RD: f64 = 287.05287;

    let e = vapor_pressure(t, rh);
    let w = 0.622 * e / (p - e);
    let r_m = RD * (1.0 + 1.6078 * w);

    p / (r_m * t)
}

/// Moist-air speed of sound (m/s)
pub fn moist_speed_of_sound(t: f64, rh: f64, p: f64) -> f64 {
    const RD: f64 = 287.05287;
    const CPD: f64 = 1004.685;
    const CPV: f64 = 1859.0;

    let e = vapor_pressure(t, rh);
    let w = 0.622 * e / (p - e);

    let r_m = RD * (1.0 + 1.6078 * w);
    let c_p = CPD + w * CPV;
    let c_v = c_p - r_m;

    let gamma_m = c_p / c_v;

    (gamma_m * r_m * t).sqrt()
}

pub fn wind_loglaw(z: f64, z_ref: f64, u_ref: f64, z0: f64) -> f64 {
    if z <= z0 {
        return 0.0;
    }
    u_ref * (z / z0).ln() / (z_ref / z0).ln()
}
