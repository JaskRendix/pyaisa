use crate::thermo;
use pyo3::prelude::*;

#[pymodule]
pub fn thermo_bindings(_py: Python, m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(potential_temperature, m)?)?;
    m.add_function(wrap_pyfunction!(moist_lapse_rate, m)?)?;
    m.add_function(wrap_pyfunction!(wet_bulb_temperature, m)?)?;
    m.add_function(wrap_pyfunction!(vapor_pressure, m)?)?;
    m.add_function(wrap_pyfunction!(mixing_ratio, m)?)?;
    m.add_function(wrap_pyfunction!(dew_point, m)?)?;
    m.add_function(wrap_pyfunction!(saturation_vapor_pressure, m)?)?;
    m.add_function(wrap_pyfunction!(virtual_temperature, m)?)?;
    m.add_function(wrap_pyfunction!(moist_air_density, m)?)?;
    m.add_function(wrap_pyfunction!(moist_speed_of_sound, m)?)?;
    m.add_function(wrap_pyfunction!(virtual_potential_temperature, m)?)?;
    m.add_function(wrap_pyfunction!(equivalent_potential_temperature, m)?)?;
    m.add_function(wrap_pyfunction!(moist_static_energy, m)?)?;
    Ok(())
}

#[pyfunction]
fn potential_temperature(t: f64, p: f64) -> f64 {
    thermo::potential_temperature(t, p)
}

#[pyfunction]
fn moist_lapse_rate(t: f64, p: f64, rh: f64) -> f64 {
    thermo::moist_lapse_rate(t, p, rh)
}

#[pyfunction]
fn wet_bulb_temperature(t: f64, rh: f64) -> f64 {
    thermo::wet_bulb_temperature(t, rh)
}

#[pyfunction]
fn saturation_vapor_pressure(t: f64) -> f64 {
    thermo::saturation_vapor_pressure(t)
}

#[pyfunction]
fn vapor_pressure(t: f64, rh: f64) -> f64 {
    thermo::vapor_pressure(t, rh)
}

#[pyfunction]
fn mixing_ratio(p: f64, e: f64) -> f64 {
    thermo::mixing_ratio(p, e)
}

#[pyfunction]
fn dew_point(e: f64) -> f64 {
    thermo::dew_point(e)
}

#[pyfunction]
fn virtual_temperature(t: f64, w: f64) -> f64 {
    thermo::virtual_temperature(t, w)
}

#[pyfunction]
fn moist_air_density(p: f64, t: f64, rh: f64) -> f64 {
    thermo::moist_air_density(p, t, rh)
}

#[pyfunction]
fn moist_speed_of_sound(isa: &crate::bindings::isa::Isa, h: f64, rh: f64) -> f64 {
    let (t, p, _rho) = isa
        .core()
        .atm_scalar(h)
        .unwrap_or((f64::NAN, f64::NAN, f64::NAN));
    thermo::moist_speed_of_sound(t, rh, p)
}

#[pyfunction]
fn virtual_potential_temperature(t: f64, p: f64, rh: f64) -> f64 {
    thermo::virtual_potential_temperature(t, p, rh)
}

#[pyfunction]
fn equivalent_potential_temperature(t: f64, p: f64, rh: f64) -> f64 {
    thermo::equivalent_potential_temperature(t, p, rh)
}

#[pyfunction]
fn moist_static_energy(t: f64, p: f64, rh: f64, z: f64) -> f64 {
    thermo::moist_static_energy(t, p, rh, z)
}
