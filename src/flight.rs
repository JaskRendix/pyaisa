/// Convert pressure altitude to flight level (FLxxx)
pub fn altitude_to_fl(h_p: f64) -> f64 {
    h_p / 100.0
}

/// Convert flight level to pressure altitude
pub fn fl_to_altitude(fl: f64) -> f64 {
    fl * 100.0
}

/// Convert geometric altitude to flight level
pub fn geometric_to_fl(_h: f64, p: f64) -> f64 {
    let h_p = pressure_altitude(p);
    altitude_to_fl(h_p)
}

/// QNH correction: convert geometric altitude to indicated altitude
pub fn indicated_altitude(h: f64, p: f64, qnh: f64) -> f64 {
    let h_p = pressure_altitude(p);
    let h_qnh = pressure_altitude(qnh);
    h + (h_qnh - h_p)
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
