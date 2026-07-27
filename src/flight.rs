use crate::math::pressure_altitude;

/// Convert pressure altitude to flight level (FLxxx)
pub fn altitude_to_fl(h_p: f64) -> f64 {
    h_p / 100.0
}

/// Convert flight level to pressure altitude
pub fn fl_to_altitude(fl: f64) -> f64 {
    fl * 100.0
}

/// Convert geometric altitude to flight level
pub fn geometric_to_fl(h: f64, p: f64) -> f64 {
    let h_p = pressure_altitude(p);
    altitude_to_fl(h_p)
}

/// QNH correction: convert geometric altitude to indicated altitude
pub fn indicated_altitude(h: f64, p: f64, qnh: f64) -> f64 {
    let h_p = pressure_altitude(p);
    let h_qnh = pressure_altitude(qnh);
    h + (h_qnh - h_p)
}
