use crate::wind;
use pyo3::prelude::*;

#[pymodule]
pub fn wind_bindings(_py: Python, m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(wind_power_law, m)?)?;
    m.add_function(wrap_pyfunction!(wind_loglaw_displaced, m)?)?;
    m.add_function(wrap_pyfunction!(wind_linear_shear, m)?)?;
    m.add_function(wrap_pyfunction!(wind_ekman, m)?)?;
    m.add_function(wrap_pyfunction!(gust, m)?)?;
    Ok(())
}

#[pyfunction]
fn wind_power_law(z: f64, z_ref: f64, u_ref: f64, alpha: f64) -> f64 {
    wind::wind_power_law(z, z_ref, u_ref, alpha)
}

#[pyfunction]
fn wind_loglaw_displaced(z: f64, z_ref: f64, u_ref: f64, z0: f64, d: f64) -> f64 {
    wind::wind_loglaw_displaced(z, z_ref, u_ref, z0, d)
}

#[pyfunction]
fn wind_linear_shear(z: f64, z0: f64, z1: f64, u0: f64, u1: f64) -> f64 {
    wind::wind_linear_shear(z, z0, z1, u0, u1)
}

#[pyfunction]
fn wind_ekman(z: f64, u0: f64, v0: f64, z_ek: f64, angle_max_deg: f64) -> (f64, f64) {
    wind::wind_ekman(z, u0, v0, z_ek, angle_max_deg)
}

#[pyfunction]
fn gust(u_mean: f64, g_factor: f64) -> f64 {
    wind::gust(u_mean, g_factor)
}
