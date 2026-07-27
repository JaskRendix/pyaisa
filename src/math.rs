/// Zero-check indicator function using standard float precision
pub fn d(x: f64) -> f64 {
    if x.abs() < f64::EPSILON {
        1.0
    } else {
        0.0
    }
}

/// Convert geometric altitude h [m] → geopotential altitude H [m]
pub fn geometric_to_geopotential(h: f64) -> f64 {
    const RE: f64 = 6356766.0; // USSA76 nominal Earth radius [m]
    (RE * h) / (RE + h)
}

/// Convert geopotential altitude H [m] → geometric altitude h [m]
pub fn geopotential_to_geometric(h_geo: f64) -> f64 {
    const RE: f64 = 6356766.0;
    if h_geo >= RE {
        return f64::INFINITY;
    }
    (RE * h_geo) / (RE - h_geo)
}

/// Dry-air speed of sound [m/s]
pub fn speed_of_sound(t: f64) -> f64 {
    const GAMMA: f64 = 1.4;
    const R: f64 = 287.05287;
    (GAMMA * R * t).sqrt()
}

/// Dynamic pressure [Pa]
pub fn dynamic_pressure(rho: f64, v: f64) -> f64 {
    0.5 * rho * v * v
}

/// Mach number
pub fn mach(v: f64, a: f64) -> f64 {
    v / a
}

/// Convert pressure p [Pa] → Pressure Altitude [m]
///
/// Strictly compliant with USSA76 across Troposphere and Stratosphere (<86 km).
pub fn pressure_altitude(p: f64) -> f64 {
    const P0: f64 = 101325.0;
    const C: f64 = 44330.769;
    C * (1.0 - (p / P0).powf(0.190263))
}

/// Convert pressure p [Pa] and temperature t [K] → Density Altitude [m]
///
/// Uses standard linear approximation around sea level.
pub fn density_altitude(p: f64, t: f64) -> f64 {
    let pa = pressure_altitude(p);
    const T_STD: f64 = 288.15;
    pa + 118.8 * (t - T_STD)
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

/// Logarithmic wind profile [m/s]
pub fn wind_loglaw(z: f64, z_ref: f64, u_ref: f64, z0: f64) -> f64 {
    if z <= z0 || z_ref <= z0 || z0 <= 0.0 {
        return 0.0;
    }
    u_ref * (z / z0).ln() / (z_ref / z0).ln()
}

pub fn dynamic_viscosity_sutherland(T: f64) -> f64 {
    // Sutherland constants for dry air
    const MU0: f64 = 1.716e-5; // reference viscosity [Pa·s]
    const T0: f64 = 273.15; // reference temperature [K]
    const S: f64 = 110.4; // Sutherland constant [K]

    MU0 * (T / T0).powf(3.0 / 2.0) * (T0 + S) / (T + S)
}

pub fn kinematic_viscosity(mu: f64, rho: f64) -> f64 {
    mu / rho
}

pub fn reynolds_number(rho: f64, v: f64, L: f64, mu: f64) -> f64 {
    rho * v * L / mu
}

pub fn stagnation_temperature(T: f64, mach: f64) -> f64 {
    const GAMMA: f64 = 1.4;
    T * (1.0 + 0.5 * (GAMMA - 1.0) * mach * mach)
}

pub fn stagnation_pressure(p: f64, mach: f64) -> f64 {
    const GAMMA: f64 = 1.4;
    p * (1.0 + 0.5 * (GAMMA - 1.0) * mach * mach).powf(GAMMA / (GAMMA - 1.0))
}

pub fn stagnation_entropy(T: f64, p: f64) -> f64 {
    const R: f64 = 287.05287;
    const CP: f64 = 1004.685;

    CP * T.ln() - R * p.ln()
}

pub fn prandtl_glauert(mach: f64) -> f64 {
    let beta2 = 1.0 - mach * mach;
    if beta2 <= 0.0 {
        return f64::INFINITY;
    }
    1.0 / beta2.sqrt()
}

pub fn eas_to_tas(eas: f64, rho: f64, rho0: f64) -> f64 {
    eas * (rho0 / rho).sqrt()
}

pub fn tas_to_eas(tas: f64, rho: f64, rho0: f64) -> f64 {
    tas * (rho / rho0).sqrt()
}

pub fn cas_to_eas(cas: f64, p0: f64, rho0: f64) -> f64 {
    const GAMMA: f64 = 1.4;
    let qc = dynamic_pressure(rho0, cas);
    let term = qc / p0 + 1.0;
    let mach = ((term.powf((GAMMA - 1.0) / GAMMA) - 1.0) * 2.0 / (GAMMA - 1.0)).sqrt();
    mach * (GAMMA * p0 / rho0).sqrt()
}

pub fn mach_from_tas(tas: f64, a: f64) -> f64 {
    tas / a
}
