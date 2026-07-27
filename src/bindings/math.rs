use crate::math;
use pyo3::prelude::*;

#[pymodule]
pub fn math_bindings(_py: Python, m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(speed_of_sound, m)?)?;
    m.add_function(wrap_pyfunction!(dynamic_pressure, m)?)?;
    m.add_function(wrap_pyfunction!(mach, m)?)?;
    m.add_function(wrap_pyfunction!(pressure_altitude, m)?)?;
    m.add_function(wrap_pyfunction!(density_altitude, m)?)?;
    m.add_function(wrap_pyfunction!(saturation_vapor_pressure, m)?)?;
    m.add_function(wrap_pyfunction!(vapor_pressure, m)?)?;
    m.add_function(wrap_pyfunction!(mixing_ratio, m)?)?;
    m.add_function(wrap_pyfunction!(dew_point, m)?)?;
    m.add_function(wrap_pyfunction!(virtual_temperature, m)?)?;
    m.add_function(wrap_pyfunction!(wind_loglaw, m)?)?;
    m.add_function(wrap_pyfunction!(geometric_to_geopotential, m)?)?;
    m.add_function(wrap_pyfunction!(geopotential_to_geometric, m)?)?;
    m.add_function(wrap_pyfunction!(moist_air_density, m)?)?;

    // 🔥 Missing function — required for Python imports
    m.add_function(wrap_pyfunction!(moist_speed_of_sound, m)?)?;

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
fn pressure_altitude(p: f64) -> f64 {
    math::pressure_altitude(p)
}

#[pyfunction]
fn density_altitude(p: f64, t: f64) -> f64 {
    math::density_altitude(p, t)
}

#[pyfunction]
fn saturation_vapor_pressure(t: f64) -> f64 {
    math::saturation_vapor_pressure(t)
}

#[pyfunction]
fn vapor_pressure(t: f64, rh: f64) -> f64 {
    math::vapor_pressure(t, rh)
}

#[pyfunction]
fn mixing_ratio(p: f64, e: f64) -> f64 {
    math::mixing_ratio(p, e)
}

#[pyfunction]
fn dew_point(e: f64) -> f64 {
    math::dew_point(e)
}

#[pyfunction]
fn virtual_temperature(t: f64, w: f64) -> f64 {
    math::virtual_temperature(t, w)
}

#[pyfunction]
fn wind_loglaw(z: f64, z_ref: f64, u_ref: f64, z0: f64) -> f64 {
    math::wind_loglaw(z, z_ref, u_ref, z0)
}

#[pyfunction]
fn geometric_to_geopotential(h: f64) -> f64 {
    math::geometric_to_geopotential(h)
}

#[pyfunction]
fn geopotential_to_geometric(h: f64) -> f64 {
    math::geopotential_to_geometric(h)
}

#[pyfunction]
fn moist_air_density(p: f64, t: f64, rh: f64) -> f64 {
    math::moist_air_density(p, t, rh)
}

#[pyfunction]
fn moist_speed_of_sound(isa: &crate::bindings::isa::ISA, h: f64, rh: f64) -> f64 {
    let (t, p, _rho) = isa
        .core()
        .atm_scalar(h)
        .unwrap_or((f64::NAN, f64::NAN, f64::NAN));
    math::moist_speed_of_sound(t, rh, p)
}
