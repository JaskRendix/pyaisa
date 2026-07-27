use crate::thermo;
use pyo3::prelude::*;

#[pymodule]
pub fn thermo_bindings(_py: Python, m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(potential_temperature, m)?)?;
    m.add_function(wrap_pyfunction!(moist_lapse_rate, m)?)?;
    m.add_function(wrap_pyfunction!(wet_bulb_temperature, m)?)?;
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
