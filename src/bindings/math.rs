use crate::math;
use pyo3::prelude::*;

#[pymodule]
pub fn math_bindings(_py: Python, m: &Bound<'_, PyModule>) -> PyResult<()> {
    // Existing bindings
    m.add_function(wrap_pyfunction!(speed_of_sound, m)?)?;
    m.add_function(wrap_pyfunction!(dynamic_pressure, m)?)?;
    m.add_function(wrap_pyfunction!(mach, m)?)?;
    m.add_function(wrap_pyfunction!(dynamic_viscosity_sutherland, m)?)?;
    m.add_function(wrap_pyfunction!(kinematic_viscosity, m)?)?;
    m.add_function(wrap_pyfunction!(reynolds_number, m)?)?;
    m.add_function(wrap_pyfunction!(stagnation_temperature, m)?)?;
    m.add_function(wrap_pyfunction!(stagnation_pressure, m)?)?;
    m.add_function(wrap_pyfunction!(stagnation_entropy, m)?)?;
    m.add_function(wrap_pyfunction!(prandtl_glauert, m)?)?;
    m.add_function(wrap_pyfunction!(eas_to_tas, m)?)?;
    m.add_function(wrap_pyfunction!(tas_to_eas, m)?)?;
    m.add_function(wrap_pyfunction!(cas_to_eas, m)?)?;
    m.add_function(wrap_pyfunction!(mach_from_tas, m)?)?;

    Ok(())
}

#[pyfunction]
fn speed_of_sound(t: f64) -> f64 {
    math::speed_of_sound(t)
}

#[pyfunction]
fn dynamic_pressure(rho: f64, v: f64) -> f64 {
    math::dynamic_pressure(rho, v)
}

#[pyfunction]
fn mach(v: f64, a: f64) -> f64 {
    math::mach(v, a)
}

#[pyfunction]
fn dynamic_viscosity_sutherland(t: f64) -> f64 {
    math::dynamic_viscosity_sutherland(t)
}

#[pyfunction]
fn kinematic_viscosity(mu: f64, rho: f64) -> f64 {
    math::kinematic_viscosity(mu, rho)
}

#[pyfunction]
fn reynolds_number(rho: f64, v: f64, l: f64, mu: f64) -> f64 {
    math::reynolds_number(rho, v, l, mu)
}

#[pyfunction]
fn stagnation_temperature(t: f64, mach: f64) -> f64 {
    math::stagnation_temperature(t, mach)
}

#[pyfunction]
fn stagnation_pressure(p: f64, mach: f64) -> f64 {
    math::stagnation_pressure(p, mach)
}

#[pyfunction]
fn stagnation_entropy(t: f64, p: f64) -> f64 {
    math::stagnation_entropy(t, p)
}

#[pyfunction]
fn prandtl_glauert(mach: f64) -> f64 {
    math::prandtl_glauert(mach)
}

#[pyfunction]
fn eas_to_tas(eas: f64, rho: f64, rho0: f64) -> f64 {
    math::eas_to_tas(eas, rho, rho0)
}

#[pyfunction]
fn tas_to_eas(tas: f64, rho: f64, rho0: f64) -> f64 {
    math::tas_to_eas(tas, rho, rho0)
}

#[pyfunction]
fn cas_to_eas(cas: f64, p0: f64, rho0: f64) -> f64 {
    math::cas_to_eas(cas, p0, rho0)
}

#[pyfunction]
fn mach_from_tas(tas: f64, a: f64) -> f64 {
    math::mach_from_tas(tas, a)
}
