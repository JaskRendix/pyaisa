//! Aircraft performance models
//!
//! Includes:
//! - Drag polars
//! - Thrust lapse
//! - Drag force
//! - Climb rate
//! - Service ceiling criteria

/// Drag polar: CD = CD0 + k * CL^2
pub fn drag_polar(cd0: f64, k: f64, cl: f64) -> f64 {
    cd0 + k * cl * cl
}

/// Thrust lapse model (simple ISA-based)
/// T = T_sl * (rho / rho_sl)
pub fn thrust_lapse(thrust_sl: f64, t: f64, p: f64) -> f64 {
    const R: f64 = 287.05287;
    let rho = p / (R * t);
    let rho_sl = 1.225;
    thrust_sl * (rho / rho_sl)
}

/// Drag force: D = q * S * CD
pub fn drag_force(q: f64, s: f64, cd: f64) -> f64 {
    q * s * cd
}

/// Climb rate: RC = (T - D) * V / W
pub fn climb_rate(thrust: f64, drag: f64, v: f64, weight: f64) -> f64 {
    (thrust - drag) * v / weight
}

/// Service ceiling condition: RC <= 0.5 m/s
pub fn service_ceiling(rc: f64) -> bool {
    rc <= 0.5
}
