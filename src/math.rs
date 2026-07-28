/// Zero-check indicator function using standard float precision
pub fn d(x: f64) -> f64 {
    if x.abs() < f64::EPSILON {
        1.0
    } else {
        0.0
    }
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

pub fn dynamic_viscosity_sutherland(t: f64) -> f64 {
    // Sutherland constants for dry air
    const MU0: f64 = 1.716e-5; // reference viscosity [Pa·s]
    const T0: f64 = 273.15; // reference temperature [K]
    const S: f64 = 110.4; // Sutherland constant [K]

    MU0 * (t / T0).powf(3.0 / 2.0) * (T0 + S) / (t + S)
}

pub fn kinematic_viscosity(mu: f64, rho: f64) -> f64 {
    mu / rho
}

pub fn reynolds_number(rho: f64, v: f64, l: f64, mu: f64) -> f64 {
    rho * v * l / mu
}

pub fn stagnation_temperature(t: f64, mach: f64) -> f64 {
    const GAMMA: f64 = 1.4;
    t * (1.0 + 0.5 * (GAMMA - 1.0) * mach * mach)
}

pub fn stagnation_pressure(p: f64, mach: f64) -> f64 {
    const GAMMA: f64 = 1.4;
    p * (1.0 + 0.5 * (GAMMA - 1.0) * mach * mach).powf(GAMMA / (GAMMA - 1.0))
}

pub fn stagnation_entropy(t: f64, p: f64) -> f64 {
    const R: f64 = 287.05287;
    const CP: f64 = 1004.685;

    CP * t.ln() - R * p.ln()
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
