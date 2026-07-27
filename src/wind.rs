/// Power-law wind profile
/// u(z) = u_ref * (z / z_ref)^alpha
/// Typical alpha:
///   - 0.14 rural
///   - 0.20 suburban
///   - 0.33 urban
pub fn wind_power_law(z: f64, z_ref: f64, u_ref: f64, alpha: f64) -> f64 {
    if z <= 0.0 || z_ref <= 0.0 {
        return 0.0;
    }
    u_ref * (z / z_ref).powf(alpha)
}

/// Logarithmic wind profile [m/s]
pub fn wind_loglaw(z: f64, z_ref: f64, u_ref: f64, z0: f64) -> f64 {
    if z <= z0 || z_ref <= z0 || z0 <= 0.0 {
        return 0.0;
    }
    u_ref * (z / z0).ln() / (z_ref / z0).ln()
}

/// Log-law with displacement height (urban/forest canopy)
/// u(z) = u_ref * ln((z - d)/z0) / ln((z_ref - d)/z0)
pub fn wind_loglaw_displaced(z: f64, z_ref: f64, u_ref: f64, z0: f64, d: f64) -> f64 {
    if z <= z0 + d || z_ref <= z0 + d {
        return 0.0;
    }
    let num = (z - d) / z0;
    let den = (z_ref - d) / z0;
    u_ref * num.ln() / den.ln()
}

/// Linear shear profile between z0 and z1
/// Useful for engineering approximations and CFD boundary conditions.
pub fn wind_linear_shear(z: f64, z0: f64, z1: f64, u0: f64, u1: f64) -> f64 {
    if z <= z0 {
        return u0;
    }
    if z >= z1 {
        return u1;
    }
    let t = (z - z0) / (z1 - z0);
    u0 + t * (u1 - u0)
}

/// Simplified Ekman spiral
/// Produces backing/veering with height.
/// Not a full Ekman solution, but useful for qualitative wind rotation.
pub fn wind_ekman(z: f64, u0: f64, v0: f64, z_ek: f64, angle_max_deg: f64) -> (f64, f64) {
    if z <= 0.0 {
        return (u0, v0);
    }

    let t = (z / z_ek).min(1.0);
    let angle = angle_max_deg.to_radians() * t;

    let speed0 = (u0 * u0 + v0 * v0).sqrt();
    let dir0 = v0.atan2(u0);

    let dir = dir0 + angle;

    let u = speed0 * dir.cos();
    let v = speed0 * dir.sin();

    (u, v)
}

/// Gust factor model
/// U_gust = U_mean * (1 + G)
pub fn gust(u_mean: f64, g_factor: f64) -> f64 {
    u_mean * (1.0 + g_factor)
}
